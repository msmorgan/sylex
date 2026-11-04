//! `property` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::Property {
    type Target = sylex_core::Property;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Ability(f0) => ability_conferral(f0.lower()),
            Self::Continuous(f0, f1) => {
                sylex_core::Property::Continuous(f0.lower(), f1.lower())
            }
            Self::StateBased { condition, effect } => sylex_core::Property::StateBased {
                condition: condition.lower(),
                effect: effect.lower(),
            },
            Self::TurnBased { at, effect } => sylex_core::Property::TurnBased {
                at: at.lower(),
                effect: effect.lower(),
            },
        }
    }
}

/// Re-home an `Ability`-flavored v1 conferral onto the core flavor its
/// CONTENT calls for. A DEONTIC row conferred by a type or subtype states a
/// quality of the object — "an Equipment can be attached to a creature"
/// ([CR#301.5]), "a land can be played" ([CR#305.9]) — and [CR#113.12] is
/// explicit that stating a quality is "neither granting an ability nor setting
/// a characteristic", so it lands on the ability-free
/// [`sylex_core::Property::Static`]. Everything else is a real conferred
/// ability ([CR#305.6] a basic land type's mana ability, [CR#714.3a] a Saga's
/// enters-with-a-lore-counter replacement) and stays `Property::Ability`.
fn ability_conferral(ability: std::sync::Arc<sylex_core::Ability>) -> sylex_core::Property {
    if let sylex_core::Ability::Static(region) = ability.as_ref() {
        // A rules-defined state-based action is a GAME ACTION, not an ability
        // of any kind ([CR#704.1,704.1a]) — the Aura must-be-attached rule
        // ([CR#704.5m]). v1 spells it as an `Sba` static (lowered to
        // `ConditionallyDo`); core's home for it is the ability-free
        // `StateBased` flavor.
        if let sylex_core::StaticSpec::ConditionallyDo { when, then } = &region.body {
            return sylex_core::Property::StateBased {
                condition: std::sync::Arc::clone(when),
                effect: std::sync::Arc::clone(then),
            };
        }
        if is_deontic(&region.body) {
            return sylex_core::Property::Static(std::sync::Arc::clone(region));
        }
    }
    sylex_core::Property::Ability(ability)
}

/// The v1 adapter for the Combatant role. v1's vocabulary has no role row, so
/// a v1 Card Type or Subtype spells the creature-like combat role the only way
/// it can — the self `May(Attack)` and `May(Block)` grants ([CR#508.1a,509.1a])
/// with the summoning-sickness `Cant` pair that rides along ([CR#302.6]). Core
/// takes the role itself ([CR#113.12]: the type states a quality of the object)
/// and DERIVES those four rules from it, so the bundle collapses to one
/// `Role(Combatant)` conferral, held while the bearer is a permanent
/// ([CR#110.1]). The whole v1 authoring path deletes at cutover, where the role
/// is written directly.
///
/// Absent the pair, every conferral passes through untouched: one grant alone
/// is an action permission and no role.
pub(crate) fn conferrals(
    confers: std::sync::Arc<[sylex_core::Property]>,
) -> std::sync::Arc<[sylex_core::Property]> {
    if !(confers.iter().any(may_attack_grant) && confers.iter().any(may_block_grant)) {
        return confers;
    }
    std::iter::once(combatant_role())
        .chain(
            confers
                .iter()
                .filter(|p| !may_attack_grant(p) && !may_block_grant(p) && !sickness_row(p))
                .cloned(),
        )
        .collect()
}

/// One `Role(Combatant)` conferral, gated on the bearer being a permanent
/// ([CR#110.1]) — the shape a v2 Card Type declaration writes directly.
fn combatant_role() -> sylex_core::Property {
    use sylex_core::Predicate;
    use sylex_core::Reference;
    sylex_core::Property::Static(std::sync::Arc::new(sylex_core::Region::candidate(
        sylex_core::StaticSpec::Conditionally(
            sylex_core::Condition::Matches(
                Reference::source_parameter(),
                Predicate::Class(sylex_core::ObjectClass::Permanent),
            ),
            std::sync::Arc::new(sylex_core::StaticSpec::Role {
                who: Predicate::Ref(Reference::source_parameter()),
                role: sylex_core::Role::Combatant,
            }),
        ),
    )))
}

/// Whether `p` is the v1 bundle's self `May(Attack)` grant.
fn may_attack_grant(p: &sylex_core::Property) -> bool {
    matches!(
        conferred_body(p),
        Some(sylex_core::StaticSpec::Deontic(sylex_core::Deontic::May(
            sylex_core::DeonticAction::Attack { by, .. },
        ))) if *by == self_ref()
    )
}

/// Whether `p` is the v1 bundle's self `May(Block)` grant.
fn may_block_grant(p: &sylex_core::Property) -> bool {
    matches!(
        conferred_body(p),
        Some(sylex_core::StaticSpec::Deontic(sylex_core::Deontic::May(
            sylex_core::DeonticAction::Block { by, .. },
        ))) if *by == self_ref()
    )
}

/// The conferral's bearer, as a row predicate.
fn self_ref() -> sylex_core::Predicate {
    sylex_core::Predicate::Ref(sylex_core::Reference::source_parameter())
}

/// Whether `p` is one of the summoning-sickness restrictions the role derives
/// ([CR#302.6]): a gated self `Cant(Attack)`, or a gated self `Cant(Activate)`
/// scoped to a tap cost ([CR#602.5a]).
fn sickness_row(p: &sylex_core::Property) -> bool {
    use sylex_core::Deontic;
    use sylex_core::DeonticAction;
    use sylex_core::StaticSpec;
    let Some(StaticSpec::Conditionally(_, inner)) = conferred_body(p) else {
        return false;
    };
    let this = self_ref();
    match inner.as_ref() {
        StaticSpec::Deontic(Deontic::Cant(DeonticAction::Attack { by, .. })) => *by == this,
        StaticSpec::Deontic(Deontic::Cant(DeonticAction::Activate { what, cost, .. })) => {
            *what == this && cost.is_some()
        }
        _ => false,
    }
}

/// The static a conferral carries, whichever flavor it landed on.
fn conferred_body(p: &sylex_core::Property) -> Option<&sylex_core::StaticSpec> {
    match p {
        sylex_core::Property::Static(region) => Some(&region.body),
        sylex_core::Property::Ability(a) => match a.as_ref() {
            sylex_core::Ability::Static(region) => Some(&region.body),
            _ => None,
        },
        _ => None,
    }
}

/// Whether a Static Spec is a deontic row, looking through the
/// `Conditionally` gate that Creature's summoning-sickness pair ([CR#302.6])
/// wears.
fn is_deontic(spec: &sylex_core::StaticSpec) -> bool {
    match spec {
        sylex_core::StaticSpec::Deontic(_) => true,
        sylex_core::StaticSpec::Conditionally(_, inner) => is_deontic(inner),
        _ => false,
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
    fn lowers_property_ability() {
        assert_matches!(
            sylex_semantics_v1::Property::Ability(std::sync::Arc::new(minimal_ability())).lower(),
            sylex_core::Property::Ability(_)
        );
    }

    /// [CR#113.12,301.5]: a DEONTIC conferral is a quality of the object, so
    /// it re-homes onto the ability-free `Property::Static` flavor rather than
    /// staying an ability.
    #[test]
    fn deontic_conferral_re_homes_to_property_static() {
        use sylex_core::Deontic;
        use sylex_core::DeonticAction;
        use sylex_core::StaticSpec;

        let rule = sylex_core::Ability::r#static(StaticSpec::Deontic(Deontic::May(
            DeonticAction::Attach {
                what: sylex_core::Predicate::Any,
                to: sylex_core::Predicate::Any,
            },
        )));
        assert_matches!(
            super::ability_conferral(std::sync::Arc::new(rule)),
            sylex_core::Property::Static(_)
        );
    }

    /// The v1 combat bundle collapses to one `Role(Combatant)` conferral: core
    /// derives the permissions and the summoning-sickness pair from the role
    /// ([CR#113.12,302.6,508.1a,509.1a]), so carrying them beside it would say
    /// the same thing twice.
    #[test]
    fn the_v1_combat_bundle_collapses_to_the_combatant_role() {
        use sylex_core::Deontic;
        use sylex_core::DeonticAction;
        use sylex_core::Predicate;
        use sylex_core::Property;
        use sylex_core::Reference;
        use sylex_core::Role;
        use sylex_core::StaticSpec;

        let this = || Predicate::Ref(Reference::source_parameter());
        let rule = |s: StaticSpec| {
            Property::Ability(std::sync::Arc::new(sylex_core::Ability::r#static(s)))
        };
        let sick = || {
            sylex_core::Condition::Matches(
                Reference::source_parameter(),
                Predicate::State(sylex_core::StatePredicate::SummoningSick),
            )
        };
        let bundle: std::sync::Arc<[Property]> = vec![
            rule(StaticSpec::Deontic(Deontic::May(DeonticAction::Attack {
                by: this(),
                on: Predicate::Any,
            }))),
            rule(StaticSpec::Deontic(Deontic::May(DeonticAction::Block {
                by: this(),
                on: Predicate::Any,
                count: None,
            }))),
            rule(StaticSpec::Conditionally(
                sick(),
                std::sync::Arc::new(StaticSpec::Deontic(Deontic::Cant(DeonticAction::Attack {
                    by: this(),
                    on: Predicate::Any,
                }))),
            )),
            rule(StaticSpec::Conditionally(
                sick(),
                std::sync::Arc::new(StaticSpec::Deontic(Deontic::Cant(
                    DeonticAction::Activate {
                        what: this(),
                        by: Predicate::Any,
                        cost: Some(sylex_core::CostPredicate::IncludesTapSymbol),
                    },
                ))),
            )),
        ]
        .into();

        let lowered = super::conferrals(bundle);
        assert_eq!(lowered.len(), 1, "four rows in, one role out");
        let Property::Static(region) = &lowered[0] else {
            panic!("the role is an ability-free rule ([CR#113.12])")
        };
        let StaticSpec::Conditionally(_, inner) = &region.body else {
            panic!("the role is held while the bearer is a permanent ([CR#110.1])")
        };
        assert_matches!(
            inner.as_ref(),
            StaticSpec::Role {
                role: Role::Combatant,
                ..
            }
        );
    }

    /// One grant alone is an action permission, not the role: the bundle rule
    /// fires on the pair or not at all.
    #[test]
    fn a_lone_block_grant_is_left_as_it_is() {
        use sylex_core::Deontic;
        use sylex_core::DeonticAction;
        use sylex_core::Predicate;
        use sylex_core::Property;
        use sylex_core::Reference;
        use sylex_core::StaticSpec;

        let lone: std::sync::Arc<[Property]> = vec![Property::Static(std::sync::Arc::new(
            sylex_core::Region::candidate(StaticSpec::Deontic(Deontic::May(
                DeonticAction::Block {
                    by: Predicate::Ref(Reference::source_parameter()),
                    on: Predicate::Any,
                    count: None,
                },
            ))),
        ))]
        .into();
        assert_eq!(super::conferrals(std::sync::Arc::clone(&lone)), lone);
    }

    #[test]
    fn lowers_property_continuous() {
        assert_matches!(
            sylex_semantics_v1::Property::Continuous(minimal_reference(), minimal_modification())
                .lower(),
            sylex_core::Property::Continuous(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Modification::Power(sylex_core::NumericOp::Set(
                    sylex_core::StatValue::DefinedByAbility
                ))
            )
        );
    }

    #[test]
    fn lowers_property_state_based() {
        assert_matches!(
            in_spell_region(|| sylex_semantics_v1::Property::StateBased {
                condition: std::sync::Arc::new(minimal_condition()),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            sylex_core::Property::StateBased {
                condition: _,
                effect: _
            }
        );
    }

    #[test]
    fn lowers_property_turn_based() {
        assert_matches!(
            in_spell_region(|| sylex_semantics_v1::Property::TurnBased {
                at: minimal_phase_step(),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            sylex_core::Property::TurnBased {
                at: sylex_core::PhaseStep::Beginning(sylex_core::BeginningStep::Untap),
                effect: _
            }
        );
    }
}
