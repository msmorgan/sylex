//! `ability` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

fn peel_targets(
    effect: sylex_semantics_v1::OneShotEffect,
) -> (
    std::sync::Arc<[sylex_semantics_v1::TargetSpec]>,
    sylex_semantics_v1::OneShotEffect,
) {
    match effect {
        sylex_semantics_v1::OneShotEffect::Targeted(targeted) => (
            targeted.targets,
            std::sync::Arc::unwrap_or_clone(targeted.effect),
        ),
        sylex_semantics_v1::OneShotEffect::Expanded(expanded) => peel_targets(*expanded.value),
        sylex_semantics_v1::OneShotEffect::May(mut may) => {
            let (targets, effect) = peel_targets(std::sync::Arc::unwrap_or_clone(may.effect));
            may.effect = std::sync::Arc::new(effect);
            (targets, sylex_semantics_v1::OneShotEffect::May(may))
        }
        other => (std::sync::Arc::from([]), other),
    }
}
/// Split an ability's effect into its announcement declarations and the body
/// that resolves: the target telescope ([CR#601.2c]) and the PRINTED
/// ADDITIONAL COST ([CR#118.8,601.2b]).
///
/// An additional cost is by definition paid with the spell's mana cost or the
/// ability's activation cost ([CR#118.8,118.8a]), so a semantic
/// `AdditionalCost` at an ability root is a DECLARATION, not an instruction:
/// it is hoisted here onto the ability's cost and never reaches the body. (A
/// payment made while the ability resolves is [CR#118.12]'s "[do something].
/// If you do, …" — core's `May`, a different node.)
fn peel_announcement(
    effect: sylex_semantics_v1::OneShotEffect,
) -> (
    Vec<sylex_semantics_v1::TargetSpec>,
    Vec<sylex_semantics_v1::CostComponent>,
    sylex_semantics_v1::OneShotEffect,
) {
    let (targets, body) = peel_targets(effect);
    let mut targets = targets.to_vec();
    match as_additional_cost(&body) {
        Some(additional) => {
            let (inner_targets, inner_pay, inner_body) = peel_announcement(
                std::sync::Arc::unwrap_or_clone(std::sync::Arc::clone(&additional.body)),
            );
            targets.extend(inner_targets);
            let mut pay = additional.pay.0.to_vec();
            pay.extend(inner_pay);
            (targets, pay, inner_body)
        }
        None => (targets, Vec::new(), body),
    }
}

/// The additional-cost declaration a spell root spells, looking through the
/// remembered macro invocation that wraps it. The wrapper's provenance does not
/// cross `lower` (spec §12) — and cannot, since core has no node for this
/// declaration at all: it becomes the ability's cost.
fn as_additional_cost(
    effect: &sylex_semantics_v1::OneShotEffect,
) -> Option<&sylex_semantics_v1::AdditionalCost> {
    match effect {
        sylex_semantics_v1::OneShotEffect::AdditionalCost(additional) => Some(additional),
        sylex_semantics_v1::OneShotEffect::Expanded(expanded) => {
            as_additional_cost(&expanded.value)
        }
        _ => None,
    }
}

/// Build an ability region together with its announcement ([CR#601.2b]).
///
/// The cost block is lowered FIRST, so its payment-time subject instructions
/// occupy the definitions that come before the body's — a paid product is a
/// def the body reads (core law 4), never an anaphor re-derived at
/// resolution. `declared` is the ability's own printed cost (an activation
/// cost, a mode's cost); a root `AdditionalCost` in `effect` is hoisted onto
/// the same block.
fn lower_announced_region(
    kind: crate::region::RegionKind,
    declared: &[sylex_semantics_v1::CostComponent],
    effect: sylex_semantics_v1::OneShotEffect,
) -> (
    std::sync::Arc<[sylex_core::TargetSpec]>,
    sylex_core::Region,
    sylex_core::Cost,
) {
    let (semantic_targets, additional, body) = peel_announcement(effect);
    let target_count = semantic_targets.len();
    let build = || {
        let targets = semantic_targets
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, target)| crate::region::with_target_prefix(index, || target.lower()))
            .collect::<Vec<_>>()
            .into();
        let mut components: Vec<sylex_semantics_v1::CostComponent> = declared.to_vec();
        components.extend(additional);
        let (cost, paid) = crate::cost::lower_cost_block(&components);
        // [CR#118.8]: the paid product IS the payment event's object, so the
        // body reads "the sacrificed creature" through the same event anaphor
        // a trigger uses — now resolved to the cost's own register.
        if let Some(paid) = paid {
            crate::region::set_event_object(paid.reference);
        }
        let body = crate::effect::lower_block(body);
        (targets, body, sylex_core::Cost(cost))
    };
    let (params, (targets, body, cost)) = if crate::region::is_active() {
        crate::region::in_carried_region(kind, target_count, build)
    } else {
        crate::region::in_region(kind, target_count, build)
    };
    (targets, sylex_core::Region::new(params, body), cost)
}

impl Lower for sylex_semantics_v1::SpellAbility {
    type Target = sylex_core::SpellAbility;
    fn lower(self) -> <Self as Lower>::Target {
        let (targets, effect, cost) =
            lower_announced_region(crate::region::RegionKind::Spell, &[], self.effect);
        sylex_core::SpellAbility {
            ability_word: self.ability_word.lower(),
            cost,
            targets,
            effect,
        }
    }
}

impl Lower for sylex_semantics_v1::ActivatedAbility {
    type Target = sylex_core::ActivatedAbility;
    fn lower(self) -> <Self as Lower>::Target {
        let (targets, effect, cost) = lower_announced_region(
            crate::region::RegionKind::Activated,
            &self.cost.0,
            self.effect,
        );
        sylex_core::ActivatedAbility {
            ability_word: self.ability_word.lower(),
            cost,
            from: self.from.lower(),
            window: self.window.lower(),
            condition: self.condition.lower(),
            limits: self.limits.lower(),
            targets,
            effect,
        }
    }
}

impl Lower for sylex_semantics_v1::UseLimit {
    type Target = sylex_core::UseLimit;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::OncePerTurn => sylex_core::UseLimit::OncePerTurn,
            Self::OncePerGame => sylex_core::UseLimit::OncePerGame,
            Self::LoyaltyOncePerTurn => sylex_core::UseLimit::LoyaltyOncePerTurn,
        }
    }
}

impl Lower for sylex_semantics_v1::TriggeredAbility {
    type Target = sylex_core::TriggeredAbility;
    fn lower(self) -> <Self as Lower>::Target {
        let sylex_semantics_v1::TriggeredAbility {
            ability_word,
            event,
            from,
            condition,
            limits,
            where_x,
            effect,
        } = self;
        let (semantic_targets, additional, body) = peel_announcement(effect);
        assert!(
            additional.is_empty(),
            "a triggered ability pays no mana cost and no activation cost, so it has \
             nowhere to pay an additional cost ([CR#118.8])"
        );
        let target_count = semantic_targets.len();
        let build = || {
            let targets = semantic_targets
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, target)| crate::region::with_target_prefix(index, || target.lower()))
                .collect::<Vec<_>>()
                .into();
            // [CR#702.21b]: the "where X is …" definition rides the ability,
            // not the body. It fills the region's declared announced-X
            // parameter at entry, so the toll's `{X}` and every `Count::X` in
            // the body are the same indexed read — never a magnitude the body
            // happened to pin most recently.
            let where_x = where_x.map(Lower::lower);
            let instructions = crate::effect::lower_block(body);
            (
                event.lower(),
                condition.lower(),
                targets,
                where_x,
                instructions,
            )
        };
        let (params, (event, condition, targets, where_x, body)) = if crate::region::is_active() {
            crate::region::in_carried_region(
                crate::region::RegionKind::Triggered,
                target_count,
                build,
            )
        } else {
            crate::region::in_region(crate::region::RegionKind::Triggered, target_count, build)
        };
        sylex_core::TriggeredAbility {
            ability_word: ability_word.lower(),
            event,
            from: from.lower(),
            condition,
            limits: limits.lower(),
            where_x,
            targets,
            effect: sylex_core::Region::new(params, body),
        }
    }
}

impl Lower for sylex_semantics_v1::ChooseSpec {
    type Target = sylex_core::ChooseSpec;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::ChooseSpec {
            count: self.count.lower(),
            up_to: self.up_to.lower(),
            repeats: self.repeats.lower(),
            chooser: self.chooser.lower(),
            rider: self.rider.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::ModalCostRider {
    type Target = sylex_core::ModalCostRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Entwine(f0) => sylex_core::ModalCostRider::Entwine(f0.lower()),
            Self::Escalate(f0) => sylex_core::ModalCostRider::Escalate(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::Mode {
    type Target = sylex_core::Mode;
    fn lower(self) -> <Self as Lower>::Target {
        let declared = self.cost.unwrap_or_else(|| std::sync::Arc::from([]));
        let (targets, effect, cost) =
            lower_announced_region(crate::region::RegionKind::Mode, &declared, self.effect);
        sylex_core::Mode {
            targets,
            effect,
            cost,
        }
    }
}

impl Lower for sylex_semantics_v1::Ability {
    type Target = sylex_core::Ability;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Static(f0) => {
                let kind = if static_observes_event(&f0) {
                    crate::region::RegionKind::StaticEvent
                } else {
                    crate::region::RegionKind::Static
                };
                let build = || std::sync::Arc::unwrap_or_clone(f0).lower();
                let (params, body) = if crate::region::is_active() {
                    crate::region::in_carried_region(kind, 0, build)
                } else {
                    crate::region::in_region(kind, 0, build)
                };
                sylex_core::Ability::Static(std::sync::Arc::new(sylex_core::Region::new(
                    params, body,
                )))
            }
            Self::Activated(f0) => sylex_core::Ability::Activated(f0.lower()),
            Self::Triggered(f0) => sylex_core::Ability::Triggered(f0.lower()),
            Self::Spell(f0) => sylex_core::Ability::Spell(f0.lower()),
            Self::Keyword(f0) => sylex_core::Ability::Keyword(f0.lower()),
            // v1's `Innate` marker has no core counterpart: a conferred
            // ability occupies the ordinary hierarchy, so the wrapper is
            // simply erased here. Where the marker sat on an ability-FREE type
            // rule, `Property::lower` re-homes the lowered `Static` onto
            // `core::Property::Static` instead.
            Self::Innate(f0) => f0.lower().as_ref().clone(),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

fn static_observes_event(effect: &sylex_semantics_v1::StaticEffect) -> bool {
    use sylex_semantics_v1::StaticEffect;

    match effect {
        StaticEffect::Replacement(_)
        | StaticEffect::Prevention(_)
        | StaticEffect::CantPrevent { .. }
        | StaticEffect::TriggerMultiplier { .. }
        | StaticEffect::CantHappen(_)
        | StaticEffect::ReplaceRoll { .. } => true,
        StaticEffect::Each(_, body) | StaticEffect::Conditionally(_, body) => {
            static_observes_event(body)
        }
        StaticEffect::Expanded(expansion) => static_observes_event(&expansion.value),
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
    fn lowers_spell_ability() {
        let lowered = sylex_semantics_v1::SpellAbility {
            ability_word: None,
            effect: minimal_one_shot_effect(),
        }
        .lower();
        assert!(lowered.targets.is_empty());
        assert_matches!(
            lowered.effect.body.as_ref(),
            [
                sylex_core::Instruction::Let(_),
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DealDamage(
                        sylex_core::Reference::Reg(sylex_core::RefId(0)),
                        sylex_core::Count::Reg(_),
                        sylex_core::Reference::Reg(sylex_core::RefId(0))
                    ),
                    ..
                }
            ]
        );
    }

    #[test]
    fn lowers_activated_ability() {
        let lowered = sylex_semantics_v1::ActivatedAbility {
            ability_word: None,
            cost: minimal_cost(),
            from: None,
            window: None,
            condition: None,
            limits: [].into(),
            effect: minimal_one_shot_effect(),
        }
        .lower();
        assert!(lowered.targets.is_empty());
        assert_matches!(
            lowered.effect.body.as_ref(),
            [
                sylex_core::Instruction::Let(_),
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DealDamage(
                        sylex_core::Reference::Reg(sylex_core::RefId(0)),
                        sylex_core::Count::Reg(_),
                        sylex_core::Reference::Reg(sylex_core::RefId(0))
                    ),
                    ..
                }
            ]
        );
    }

    #[test]
    fn lowers_use_limit_once_per_turn() {
        assert_matches!(
            sylex_semantics_v1::UseLimit::OncePerTurn.lower(),
            sylex_core::UseLimit::OncePerTurn
        );
    }

    #[test]
    fn lowers_use_limit_once_per_game() {
        assert_matches!(
            sylex_semantics_v1::UseLimit::OncePerGame.lower(),
            sylex_core::UseLimit::OncePerGame
        );
    }

    #[test]
    fn lowers_use_limit_loyalty_once_per_turn() {
        assert_matches!(
            sylex_semantics_v1::UseLimit::LoyaltyOncePerTurn.lower(),
            sylex_core::UseLimit::LoyaltyOncePerTurn
        );
    }

    #[test]
    fn lowers_triggered_ability() {
        let lowered = sylex_semantics_v1::TriggeredAbility {
            ability_word: None,
            event: minimal_event_filter(),
            from: None,
            condition: None,
            limits: [].into(),
            where_x: None,
            effect: minimal_one_shot_effect(),
        }
        .lower();
        assert!(lowered.targets.is_empty());
        assert_matches!(
            lowered.effect.body.as_ref(),
            [
                sylex_core::Instruction::Let(_),
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DealDamage(
                        sylex_core::Reference::Reg(sylex_core::RefId(0)),
                        sylex_core::Count::Reg(_),
                        sylex_core::Reference::Reg(sylex_core::RefId(0))
                    ),
                    ..
                }
            ]
        );
    }

    #[test]
    fn lowers_choose_spec() {
        assert_matches!(
            sylex_semantics_v1::ChooseSpec {
                count: minimal_quantity(),
                up_to: false,
                repeats: false,
                chooser: minimal_reference(),
                rider: None
            }
            .lower(),
            sylex_core::ChooseSpec {
                count: sylex_core::Quantity::Range(None, None),
                up_to: false,
                repeats: false,
                chooser: sylex_core::Reference::Reg(sylex_core::RefId(0)),
                rider: None
            }
        );
    }

    #[test]
    fn lowers_modal_cost_rider_entwine() {
        assert_matches!(
            sylex_semantics_v1::ModalCostRider::Entwine(minimal_cost()).lower(),
            sylex_core::ModalCostRider::Entwine(sylex_core::Cost(_))
        );
    }

    #[test]
    fn lowers_modal_cost_rider_escalate() {
        assert_matches!(
            sylex_semantics_v1::ModalCostRider::Escalate(minimal_cost()).lower(),
            sylex_core::ModalCostRider::Escalate(sylex_core::Cost(_))
        );
    }

    #[test]
    fn lowers_mode() {
        let lowered = sylex_semantics_v1::Mode {
            effect: minimal_one_shot_effect(),
            cost: None,
        }
        .lower();
        assert!(lowered.targets.is_empty());
        assert_matches!(
            lowered.effect.body.as_ref(),
            [
                sylex_core::Instruction::Let(_),
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DealDamage(
                        sylex_core::Reference::Reg(sylex_core::RefId(0)),
                        sylex_core::Count::Reg(_),
                        sylex_core::Reference::Reg(sylex_core::RefId(0))
                    ),
                    ..
                }
            ]
        );
    }

    #[test]
    fn two_same_sort_targets_lower_to_distinct_region_parameters() {
        let effect = sylex_semantics_v1::OneShotEffect::Targeted(sylex_semantics_v1::Targeted {
            targets: vec![minimal_target_spec(), minimal_target_spec()].into(),
            effect: std::sync::Arc::new(sylex_semantics_v1::OneShotEffect::Sequentially(
                vec![
                    sylex_semantics_v1::OneShotEffect::Act(sylex_semantics_v1::Action::DrawCard(
                        sylex_semantics_v1::Reference::Target(0),
                    )),
                    sylex_semantics_v1::OneShotEffect::Act(sylex_semantics_v1::Action::DrawCard(
                        sylex_semantics_v1::Reference::Target(1),
                    )),
                ]
                .into(),
            )),
        });
        let lowered = sylex_semantics_v1::SpellAbility {
            ability_word: None,
            effect,
        }
        .lower();

        assert_eq!(lowered.targets.len(), 2);
        assert_matches!(
            lowered.effect.body.as_ref(),
            [
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DrawCard(sylex_core::Reference::Reg(
                        sylex_core::RefId(2)
                    )),
                    ..
                },
                sylex_core::Instruction::Let(_),
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DrawCard(sylex_core::Reference::Reg(
                        sylex_core::RefId(3)
                    )),
                    ..
                },
                sylex_core::Instruction::Let(_)
            ]
        );
        assert_eq!(
            sylex_core::validate_telescope(&lowered.effect, &lowered.targets),
            Ok(())
        );
    }

    #[test]
    fn trigger_event_roles_lower_to_the_fixed_region_prefix() {
        let mut ability = minimal_triggered_ability();
        ability.effect = sylex_semantics_v1::OneShotEffect::Sequentially(
            vec![
                sylex_semantics_v1::OneShotEffect::Act(sylex_semantics_v1::Action::DealDamage(
                    sylex_semantics_v1::Reference::EventObject,
                    sylex_semantics_v1::Count::Literal(1),
                    sylex_semantics_v1::Reference::EventPatient,
                )),
                sylex_semantics_v1::OneShotEffect::Act(sylex_semantics_v1::Action::DrawCard(
                    sylex_semantics_v1::Reference::EventActor,
                )),
            ]
            .into(),
        );
        let lowered = ability.lower();

        assert_matches!(
            lowered.effect.body.as_ref(),
            [
                sylex_core::Instruction::Let(_),
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DealDamage(
                        sylex_core::Reference::Reg(sylex_core::RefId(2)),
                        _,
                        sylex_core::Reference::Reg(sylex_core::RefId(3))
                    ),
                    ..
                },
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DrawCard(sylex_core::Reference::Reg(
                        sylex_core::RefId(4)
                    )),
                    ..
                },
                sylex_core::Instruction::Let(_)
            ]
        );
        assert_eq!(
            sylex_core::validate_telescope(&lowered.effect, &lowered.targets),
            Ok(())
        );
    }

    #[test]
    fn modal_and_delayed_regions_own_their_target_telescopes() {
        let targeted_mode = sylex_semantics_v1::Mode {
            effect: sylex_semantics_v1::OneShotEffect::Targeted(sylex_semantics_v1::Targeted {
                targets: vec![minimal_target_spec()].into(),
                effect: std::sync::Arc::new(sylex_semantics_v1::OneShotEffect::Act(
                    sylex_semantics_v1::Action::DrawCard(sylex_semantics_v1::Reference::Target(0)),
                )),
            }),
            cost: None,
        }
        .lower();
        assert_eq!(targeted_mode.targets.len(), 1);
        assert_matches!(
            targeted_mode.effect.body.as_ref(),
            [
                sylex_core::Instruction::Act {
                    action: sylex_core::Action::DrawCard(sylex_core::Reference::Reg(
                        sylex_core::RefId(2)
                    )),
                    ..
                },
                sylex_core::Instruction::Let(_)
            ]
        );

        let delayed = sylex_semantics_v1::OneShotEffect::Delayed(std::sync::Arc::new(
            minimal_triggered_ability(),
        ));
        let outer = sylex_semantics_v1::SpellAbility {
            ability_word: None,
            effect: sylex_semantics_v1::OneShotEffect::Targeted(sylex_semantics_v1::Targeted {
                targets: vec![minimal_target_spec()].into(),
                effect: std::sync::Arc::new(delayed),
            }),
        }
        .lower();
        let [sylex_core::Instruction::Delayed(inner)] = outer.effect.body.as_ref() else {
            panic!("outer targeted spell should contain one delayed trigger")
        };
        assert!(inner.targets.is_empty());
        assert_eq!(
            inner
                .effect
                .params
                .iter()
                .filter(|param| matches!(
                    param.provenance,
                    sylex_core::Provenance::AnnouncedTarget(_)
                ))
                .count(),
            0
        );
        assert_eq!(
            sylex_core::validate_telescope(&outer.effect, &outer.targets),
            Ok(())
        );
    }

    #[test]
    fn lowers_ability_static() {
        assert_matches!(
            sylex_semantics_v1::Ability::Static(std::sync::Arc::new(minimal_static_effect()))
                .lower(),
            sylex_core::Ability::Static(_)
        );
    }

    #[test]
    fn lowers_ability_activated() {
        assert_matches!(
            sylex_semantics_v1::Ability::Activated(
                std::sync::Arc::new(minimal_activated_ability())
            )
            .lower(),
            sylex_core::Ability::Activated(_)
        );
    }

    #[test]
    fn lowers_ability_triggered() {
        assert_matches!(
            sylex_semantics_v1::Ability::Triggered(
                std::sync::Arc::new(minimal_triggered_ability())
            )
            .lower(),
            sylex_core::Ability::Triggered(_)
        );
    }

    #[test]
    fn lowers_ability_spell() {
        assert_matches!(
            sylex_semantics_v1::Ability::Spell(std::sync::Arc::new(minimal_spell_ability()))
                .lower(),
            sylex_core::Ability::Spell(_)
        );
    }

    #[test]
    fn lowers_ability_keyword() {
        assert_matches!(
            sylex_semantics_v1::Ability::Keyword(minimal_keyword_ability()).lower(),
            sylex_core::Ability::Keyword(sylex_core::KeywordAbility::FirstStrike)
        );
    }

    /// v1's `Innate` wrapper is ERASED at lowering — the inner ability is
    /// what reaches core, in the ordinary ability hierarchy.
    #[test]
    fn lowers_ability_innate_to_its_inner_ability() {
        assert_eq!(
            sylex_semantics_v1::Ability::Innate(std::sync::Arc::new(minimal_ability())).lower(),
            minimal_ability().lower()
        );
    }

    #[test]
    fn lowers_ability_expanded() {
        assert_matches!(
            sylex_semantics_v1::Ability::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_ability())
            })
            .lower(),
            sylex_core::Ability::Static(_)
        );
    }

    fn semantic_mana_effect() -> sylex_semantics_v1::OneShotEffect {
        sylex_semantics_v1::OneShotEffect::Act(sylex_semantics_v1::Action::AddMana(
            sylex_semantics_v1::Reference::You,
            sylex_semantics_v1::Count::Literal(1),
            sylex_semantics_v1::ManaProduction::from(sylex_semantics_v1::Color::Green),
        ))
    }

    fn semantic_mana_activation(
        effect: sylex_semantics_v1::OneShotEffect,
    ) -> sylex_semantics_v1::Ability {
        let mut ability = minimal_activated_ability();
        ability.cost =
            sylex_semantics_v1::Cost(vec![sylex_semantics_v1::CostComponent::Tap].into());
        ability.effect = effect;
        sylex_semantics_v1::Ability::Activated(std::sync::Arc::new(ability))
    }

    #[test]
    fn classifies_targetless_activated_mana_ability_while_lowering() {
        let lowered = semantic_mana_activation(semantic_mana_effect()).lower();
        assert_eq!(
            lowered.mana_profile(),
            Some(sylex_core::ActivatedManaProfile::Always)
        );
        assert_matches!(lowered, sylex_core::Ability::Activated(_));
    }

    #[test]
    fn reversibility_does_not_change_mana_classification() {
        let effect = sylex_semantics_v1::OneShotEffect::Sequentially(
            vec![
                semantic_mana_effect(),
                sylex_semantics_v1::OneShotEffect::Act(sylex_semantics_v1::Action::DrawCard(
                    sylex_semantics_v1::Reference::You,
                )),
            ]
            .into(),
        );

        assert!(
            semantic_mana_activation(effect)
                .lower()
                .is_activated_mana_ability()
        );
    }

    #[test]
    fn mana_produced_by_a_binder_participates_in_classification() {
        let effect = sylex_semantics_v1::OneShotEffect::With(sylex_semantics_v1::With {
            binder: sylex_semantics_v1::Binder::Produce(std::sync::Arc::new(
                sylex_semantics_v1::Action::AddMana(
                    sylex_semantics_v1::Reference::You,
                    sylex_semantics_v1::Count::Literal(1),
                    sylex_semantics_v1::ManaProduction::from(sylex_semantics_v1::Color::Green),
                ),
            )),
            body: std::sync::Arc::new(minimal_one_shot_effect()),
        });

        assert!(
            semantic_mana_activation(effect)
                .lower()
                .is_activated_mana_ability()
        );
    }

    #[test]
    fn inert_reveal_until_body_does_not_establish_mana_production() {
        let effect =
            sylex_semantics_v1::OneShotEffect::RevealUntil(sylex_semantics_v1::RevealUntil {
                whose: sylex_semantics_v1::Reference::You,
                matches: sylex_semantics_v1::Predicate::Any,
                body: std::sync::Arc::new(semantic_mana_effect()),
            });

        assert!(
            !semantic_mana_activation(effect)
                .lower()
                .is_activated_mana_ability()
        );
    }

    #[test]
    fn does_not_classify_targeted_or_loyalty_mana_producers() {
        let targeted = sylex_semantics_v1::OneShotEffect::Targeted(sylex_semantics_v1::Targeted {
            targets: vec![minimal_target_spec()].into(),
            effect: std::sync::Arc::new(semantic_mana_effect()),
        });
        assert!(
            !semantic_mana_activation(targeted)
                .lower()
                .is_activated_mana_ability()
        );

        let sylex_semantics_v1::Ability::Activated(mut loyalty) =
            semantic_mana_activation(semantic_mana_effect())
        else {
            unreachable!()
        };
        std::sync::Arc::make_mut(&mut loyalty).limits =
            vec![sylex_semantics_v1::UseLimit::LoyaltyOncePerTurn].into();
        assert!(
            !sylex_semantics_v1::Ability::Activated(loyalty)
                .lower()
                .is_activated_mana_ability()
        );
    }

    #[test]
    fn compiles_modal_mana_profile_for_announced_modes() {
        let targeted = sylex_semantics_v1::OneShotEffect::Targeted(sylex_semantics_v1::Targeted {
            targets: vec![minimal_target_spec()].into(),
            effect: std::sync::Arc::new(semantic_mana_effect()),
        });
        let modal = sylex_semantics_v1::OneShotEffect::Modal(sylex_semantics_v1::Modal {
            choose: minimal_choose_spec(),
            modes: vec![
                sylex_semantics_v1::Mode {
                    effect: semantic_mana_effect(),
                    cost: None,
                },
                sylex_semantics_v1::Mode {
                    effect: targeted,
                    cost: None,
                },
            ]
            .into(),
        });
        let Some(sylex_core::ActivatedManaProfile::ByAnnouncedMode(classes)) =
            semantic_mana_activation(modal).lower().mana_profile()
        else {
            panic!("a modal producer with a qualifying mode needs a derived profile");
        };
        assert_eq!(classes.len(), 2);
        assert!(classes[0].adds_mana && classes[0].targetless);
        assert!(classes[1].adds_mana && !classes[1].targetless);
    }

    #[test]
    fn modal_mana_profile_does_not_treat_reversibility_as_classification() {
        let unsafe_cost = vec![sylex_semantics_v1::CostComponent::do_action(
            sylex_semantics_v1::Action::Move(
                sylex_semantics_v1::Reference::This,
                sylex_semantics_v1::Destination::Library(sylex_semantics_v1::Anchor::FromBottom(
                    sylex_semantics_v1::Count::Literal(0),
                )),
                [].into(),
                Some(sylex_semantics_v1::Zone::Battlefield),
            ),
        )]
        .into();
        let modal = sylex_semantics_v1::OneShotEffect::Modal(sylex_semantics_v1::Modal {
            choose: minimal_choose_spec(),
            modes: vec![
                sylex_semantics_v1::Mode {
                    effect: semantic_mana_effect(),
                    cost: None,
                },
                sylex_semantics_v1::Mode {
                    effect: semantic_mana_effect(),
                    cost: Some(unsafe_cost),
                },
            ]
            .into(),
        });
        let lowered = semantic_mana_activation(modal).lower();
        let Some(sylex_core::ActivatedManaProfile::ByAnnouncedMode(classes)) =
            lowered.mana_profile()
        else {
            panic!("runtime barriers do not affect mana-ability classification");
        };

        assert!(
            classes
                .iter()
                .all(|class| class.adds_mana && class.targetless)
        );
        assert!(lowered.mana_profile_for_modes(&[1]));
    }

    #[test]
    fn classifies_each_causal_triggered_mana_ability_while_lowering() {
        let any = || sylex_semantics_v1::Predicate::Any;
        let causes = [
            sylex_semantics_v1::EventFilter::ManaAbilityActivated {
                what: any(),
                by: any(),
            },
            sylex_semantics_v1::EventFilter::ManaProduced {
                what: any(),
                by: any(),
            },
            sylex_semantics_v1::EventFilter::ManaAdded {
                what: any(),
                by: any(),
            },
            sylex_semantics_v1::EventFilter::TapForMana {
                what: any(),
                by: any(),
            },
        ];
        for event in causes {
            let mut triggered = minimal_triggered_ability();
            triggered.event = event.clone();
            triggered.effect = semantic_mana_effect();
            assert!(
                sylex_semantics_v1::Ability::Triggered(std::sync::Arc::new(triggered))
                    .lower()
                    .is_triggered_mana_ability(),
                "cause {event:?}"
            );
        }
    }

    #[test]
    fn classifies_sacrifice_cost_mana_ability_without_external_replacement_effects() {
        let mut ability = minimal_activated_ability();
        ability.cost = sylex_semantics_v1::Cost(
            vec![sylex_semantics_v1::CostComponent::With {
                binder: std::sync::Arc::new(sylex_semantics_v1::Binder::ChooseOne {
                    filter: sylex_semantics_v1::Predicate::Any,
                    by: sylex_semantics_v1::Reference::You,
                }),
                body: sylex_semantics_v1::Cost(
                    vec![sylex_semantics_v1::CostComponent::do_action(
                        sylex_semantics_v1::Action::Sacrifice(
                            sylex_semantics_v1::Reference::You,
                            sylex_semantics_v1::Reference::That(sylex_semantics_v1::Sort::Card),
                        ),
                    )]
                    .into(),
                ),
            }]
            .into(),
        );
        ability.effect = semantic_mana_effect();

        assert_eq!(
            sylex_semantics_v1::Ability::Activated(std::sync::Arc::new(ability))
                .lower()
                .mana_profile(),
            Some(sylex_core::ActivatedManaProfile::Always)
        );
    }
}
