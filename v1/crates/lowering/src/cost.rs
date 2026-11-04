//! `cost` — semantics grammar to engine AST.
//!
//! A semantic cost is a list of payment demands; a core cost is a BLOCK of
//! cost instructions run at announcement in the ability's own activation
//! ([CR#601.2b,601.2h]). Lowering is what turns one into the other: a
//! semantic `With` binder becomes the subject instruction that writes the
//! payment register, and the verbs that spend it follow flat in the same
//! block, reading that register. The binding stays live afterwards, so the
//! ability body can read the paid product ("the sacrificed creature") as a
//! def rather than re-deriving it.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use std::sync::Arc;

use crate::Lower;
use crate::effect::BoundValue;

/// Lower a semantic cost list into a core cost block. The block's definitions
/// continue the ability region's sequence ([CR#601.2b]: the announcement runs
/// in the ability's own activation), but its ANAPHORA are scoped to the block —
/// a payment subject is read by the verbs that spend it, not by the effect
/// clause that follows. Returns the block plus the LAST payment subject it
/// bound: the paid product an ability body names ("the sacrificed creature",
/// [CR#118.8]), which the caller wires to the region's event-object channel.
pub(crate) fn lower_cost_block(
    components: &[sylex_semantics_v1::CostComponent],
) -> (Arc<[sylex_core::CostComponent]>, Option<BoundValue>) {
    if crate::region::is_active() {
        crate::region::scoped_anaphora(|| lower_cost_block_inner(components))
    } else {
        lower_cost_block_inner(components)
    }
}

fn lower_cost_block_inner(
    components: &[sylex_semantics_v1::CostComponent],
) -> (Arc<[sylex_core::CostComponent]>, Option<BoundValue>) {
    let mut out = Vec::new();
    let mut paid = None;
    for component in components {
        if let Some(bound) = lower_cost_component(component.clone(), &mut out) {
            paid = Some(bound);
        }
    }
    (out.into(), paid)
}

/// Lower a cost list whose bindings must NOT escape — a deontic gate, an
/// alternative or optional cost row, a modal cost rider. These are announced
/// against another announcement's register file, so nothing downstream may
/// read their products.
pub(crate) fn lower_scoped_cost_components(
    components: &[sylex_semantics_v1::CostComponent],
) -> Arc<[sylex_core::CostComponent]> {
    if crate::region::is_active() {
        crate::region::scoped_antecedents(|| lower_cost_block(components).0)
    } else {
        lower_cost_block(components).0
    }
}

/// Lower one semantic component, appending the instructions it becomes.
/// Returns the payment subject it bound, if it bound one.
fn lower_cost_component(
    component: sylex_semantics_v1::CostComponent,
    out: &mut Vec<sylex_core::CostComponent>,
) -> Option<BoundValue> {
    use sylex_semantics_v1::CostComponent as S;
    match component {
        S::Mana(f0) => {
            out.push(sylex_core::CostComponent::Mana(f0.lower()));
            None
        }
        S::ManaCostOf(f0) => {
            out.push(sylex_core::CostComponent::ManaCostOf(f0.lower()));
            None
        }
        S::Tap => {
            out.push(sylex_core::CostComponent::Tap);
            None
        }
        S::Untap => {
            out.push(sylex_core::CostComponent::Untap);
            None
        }
        S::Do(action) => lower_action_cost(&action, out),
        S::Cost(nested) => {
            // A macro list-splice ([CR#702.29a] cycling): the nested block's
            // instructions are this block's instructions, so splice rather
            // than nest — one definition sequence, one announcement.
            let (inner, bound) = lower_cost_block_inner(&nested.0);
            out.extend(inner.iter().cloned());
            bound
        }
        S::TapTotal {
            stat,
            cmp,
            count,
            filter,
        } => {
            out.push(sylex_core::CostComponent::TapTotal {
                stat: stat.lower(),
                cmp: cmp.lower(),
                count: count.lower(),
                filter: filter.lower(),
            });
            None
        }
        S::With { binder, body } => {
            // [CR#601.2b]: the choice is its OWN instruction, made before the
            // verbs that spend it, writing a register they read.
            let (instructions, bound) = lower_cost_binder(Arc::unwrap_or_clone(binder));
            out.extend(instructions);
            crate::region::push_antecedent(
                bound.reference,
                bound.kind,
                bound.cardinality,
                bound.sort,
                crate::region::Site::ExecutionFrame,
            );
            let (inner, inner_bound) = lower_cost_block_inner(&body.0);
            out.extend(inner.iter().cloned());
            Some(inner_bound.unwrap_or(bound))
        }
        // Invocation provenance does not cross `lower`: the core grammar is
        // a compiled artifact and carries no record of the semantic
        // spelling (spec §12). Prose recovers the semantic term through the
        // provenance index instead. This is the divergence ledger's first
        // non-identity arm family.
        S::Expanded(f0) => lower_cost_component(*f0.value, out),
    }
}

/// One lowered instruction as the cost instruction it becomes. A cost block
/// admits exactly the instruction kinds a payment can perform: a payment-time
/// decision and a paying action ([CR#601.2b,601.2h]).
fn cost_instruction(instruction: sylex_core::Instruction) -> sylex_core::CostComponent {
    use sylex_core::CostComponent;
    use sylex_core::Instruction;
    match instruction {
        Instruction::Choose(choice) => CostComponent::Choose(choice),
        Instruction::Search(search) => CostComponent::Search(search),
        Instruction::Let(binding) => CostComponent::Let(binding),
        Instruction::Act { dest, action } => CostComponent::Act {
            dest,
            action: runnable_action(action),
        },
        other => unreachable!(
            "a cost binder lowers to a decision or a paying action ([CR#601.2b]), got {other:?}"
        ),
    }
}

/// Lower a semantic payment binder into the cost instruction that writes its
/// subject. Random sampling is cost-specific: it is an instruction here, while
/// effect-side random iteration remains owned by `Each`.
fn lower_cost_binder(
    binder: sylex_semantics_v1::Binder,
) -> (Vec<sylex_core::CostComponent>, BoundValue) {
    match binder {
        sylex_semantics_v1::Binder::Existing(sylex_semantics_v1::Selection::Random(
            quantity,
            filter,
        )) => {
            let quantity = quantity.lower();
            let filter = Arc::new(crate::region::predicate_region(|| filter.lower()));
            let dest = crate::region::define(sylex_core::Kind::Entities);
            (
                vec![sylex_core::CostComponent::Sample(
                    sylex_core::Sample {
                        dest,
                        quantity,
                        filter,
                    },
                )],
                BoundValue {
                    reference: dest.into(),
                    kind: sylex_core::Kind::Entities,
                    cardinality: crate::region::Cardinality::Many,
                    sort: None,
                },
            )
        }
        other => {
            let (instructions, bound) = crate::effect::lower_binder(other);
            (
                instructions.into_iter().map(cost_instruction).collect(),
                bound,
            )
        }
    }
}

fn lower_action_cost(
    action: &Arc<sylex_semantics_v1::Action>,
    out: &mut Vec<sylex_core::CostComponent>,
) -> Option<BoundValue> {
    use sylex_semantics_v1::Action;
    use sylex_semantics_v1::OneShotEffect;

    let Action::Composite { name, body } = action.as_ref() else {
        out.push(sylex_core::CostComponent::Act {
            dest: None,
            action: runnable_action(action.as_ref().clone().lower()),
        });
        return None;
    };
    let OneShotEffect::With(with) = body.as_ref() else {
        out.push(sylex_core::CostComponent::Act {
            dest: None,
            action: runnable_action(action.as_ref().clone().lower()),
        });
        return None;
    };
    // "Discard a card:" — the composite's own chooser is lifted out of the
    // verb into a preceding cost instruction ([CR#701.9,601.2b]).
    let (instructions, bound) = lower_cost_binder(with.binder.clone());
    out.extend(instructions);
    crate::region::push_antecedent(
        bound.reference,
        bound.kind,
        bound.cardinality,
        bound.sort,
        crate::region::Site::ExecutionFrame,
    );
    let lowered = sylex_core::Action::Composite {
        name: (*name).lower(),
        body: with.body.clone().lower(),
    };
    out.push(sylex_core::CostComponent::Act {
        dest: None,
        action: runnable_action(lowered),
    });
    Some(bound)
}

fn runnable_action(action: sylex_core::Action) -> sylex_core::RunnableCostAction {
    sylex_core::RunnableCostAction::try_new(action)
        .expect("validated semantic cost actions must be eligible and have every choice lifted")
}

impl Lower for sylex_semantics_v1::Cost {
    type Target = sylex_core::Cost;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::Cost(lower_scoped_cost_components(&self.0))
    }
}

impl Lower for sylex_semantics_v1::CostTag {
    type Target = sylex_core::CostTag;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::CostTag(self.0.lower())
    }
}

impl Lower for sylex_semantics_v1::OptionalCost {
    type Target = sylex_core::OptionalCost;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::OptionalCost {
            components: lower_scoped_cost_components(&self.components),
            tag: self.tag.lower(),
            repeatable: self.repeatable.lower(),
        }
    }
}

impl Lower for sylex_semantics_v1::TotalCost {
    type Target = sylex_core::TotalCost;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::TotalCost {
            base: lower_scoped_cost_components(&self.base),
            trace: self.trace.lower(),
            locked: self.locked.lower(),
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
    fn lowers_cost_component_mana() {
        let (lowered, _) = super::lower_cost_block(&[sylex_semantics_v1::CostComponent::Mana(
            sylex_semantics_v1::ManaCost::from(
                std::sync::Arc::<[sylex_semantics_v1::ManaSymbol]>::from([]),
            ),
        )]);
        assert_matches!(lowered[0], sylex_core::CostComponent::Mana(_));
    }

    #[test]
    fn lowers_cost_component_mana_cost_of() {
        let (lowered, _) =
            super::lower_cost_block(&[sylex_semantics_v1::CostComponent::ManaCostOf(
                minimal_reference(),
            )]);
        assert_matches!(
            lowered[0],
            sylex_core::CostComponent::ManaCostOf(sylex_core::Reference::Reg(
                sylex_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_cost_component_tap() {
        let (lowered, _) = super::lower_cost_block(&[sylex_semantics_v1::CostComponent::Tap]);
        assert_eq!(&*lowered, &[sylex_core::CostComponent::Tap]);
    }

    #[test]
    fn lowers_cost_component_untap() {
        let (lowered, _) = super::lower_cost_block(&[sylex_semantics_v1::CostComponent::Untap]);
        assert_eq!(&*lowered, &[sylex_core::CostComponent::Untap]);
    }

    #[test]
    fn lowers_cost_component_do_to_runnable_act() {
        let action = sylex_semantics_v1::Action::Sacrifice(
            sylex_semantics_v1::Reference::You,
            sylex_semantics_v1::Reference::This,
        );
        let (lowered, paid) = super::lower_cost_block(&[sylex_semantics_v1::CostComponent::Do(
            std::sync::Arc::new(action),
        )]);
        assert_matches!(
            lowered[0],
            sylex_core::CostComponent::Act { dest: None, .. }
        );
        assert!(paid.is_none(), "a bound verb binds no new payment subject");
    }

    /// A nested cost list SPLICES: one announcement, one definition sequence
    /// ([CR#702.29a] cycling).
    #[test]
    fn lowers_nested_cost_flat() {
        let (lowered, _) = super::lower_cost_block(&[sylex_semantics_v1::CostComponent::Cost(
            sylex_semantics_v1::Cost(vec![sylex_semantics_v1::CostComponent::Tap].into()),
        )]);
        assert_eq!(&*lowered, &[sylex_core::CostComponent::Tap]);
    }

    #[test]
    fn lowers_cost_component_tap_total() {
        let (lowered, _) =
            super::lower_cost_block(&[sylex_semantics_v1::CostComponent::TapTotal {
                stat: minimal_stat(),
                cmp: minimal_cmp(),
                count: minimal_count(),
                filter: std::sync::Arc::new(minimal_predicate()),
            }]);
        assert_matches!(lowered[0], sylex_core::CostComponent::TapTotal { .. });
    }

    /// A cost-side `With` becomes a payment-time DECISION instruction followed
    /// by the verbs that spend its register ([CR#601.2b]) — the binder never
    /// survives as a nested cost.
    #[test]
    fn lowers_cost_component_with_to_a_decision_then_its_verbs() {
        let (_, (lowered, paid)) =
            crate::region::in_region(crate::region::RegionKind::Activated, 0, || {
                super::lower_cost_block(&[sylex_semantics_v1::CostComponent::With {
                    binder: std::sync::Arc::new(sylex_semantics_v1::Binder::ChooseOne {
                        filter: sylex_semantics_v1::Predicate::Any,
                        by: sylex_semantics_v1::Reference::You,
                    }),
                    body: sylex_semantics_v1::Cost(
                        vec![sylex_semantics_v1::CostComponent::Do(std::sync::Arc::new(
                            sylex_semantics_v1::Action::Sacrifice(
                                sylex_semantics_v1::Reference::You,
                                sylex_semantics_v1::Reference::That(
                                    sylex_semantics_v1::Sort::Permanent,
                                ),
                            ),
                        ))]
                        .into(),
                    ),
                }])
            });
        assert_matches!(lowered[0], sylex_core::CostComponent::Choose(_));
        assert_matches!(
            lowered[1],
            sylex_core::CostComponent::Act { dest: None, .. }
        );
        let chosen = paid
            .expect("the chooser bound the payment subject")
            .reference;
        let sylex_core::CostComponent::Act { action, .. } = &lowered[1] else {
            panic!("the paying verb is an Act");
        };
        assert_eq!(
            **action,
            sylex_core::Action::Sacrifice(
                sylex_core::Reference::Reg(sylex_core::RefId(1)),
                sylex_core::Reference::Reg(chosen),
            ),
            "the verb pays through the register the choice wrote"
        );
    }

    /// One payment binder, lowered as the cost instruction it becomes.
    /// Payment binders lower inside a region (their destinations continue the
    /// ability's definition sequence), so each case runs in one.
    fn lower_payment_binder(
        binder: sylex_semantics_v1::Binder,
    ) -> Vec<sylex_core::CostComponent> {
        let (_, (lowered, _)) =
            crate::region::in_region(crate::region::RegionKind::Activated, 0, || {
                super::lower_cost_block(&[sylex_semantics_v1::CostComponent::With {
                    binder: std::sync::Arc::new(binder),
                    body: sylex_semantics_v1::Cost([].into()),
                }])
            });
        lowered.to_vec()
    }

    // Each payment binder's lowered image. Re-spelled from the deleted
    // `Binder -> CostBinder` cases: `CostBinder` is gone, so a binder's image
    // is the cost INSTRUCTION that writes the payment subject's register
    // ([CR#601.2b]) — same binder, same asserted shape.

    /// The predicate a payment-time subject instruction filters by is a
    /// per-candidate REGION; this reads its body and checks it declares its
    /// candidate as parameter zero.
    fn pins_minimal_filter(filter: &sylex_core::Region<sylex_core::Predicate>) -> bool {
        filter.body == sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
            && matches!(
                filter.params[0].provenance,
                sylex_core::Provenance::Candidate(_)
            )
            && filter.params[0].kind == sylex_core::Kind::Entity
    }

    /// An `Activated` region declares source(0), controller(1), announced X(2),
    /// so a payment binder's own definition is register 3.
    const FIRST_PAYMENT_DEF: sylex_core::DefId = sylex_core::DefId(3);

    #[test]
    fn lowers_binder_the_ref() {
        assert_eq!(
            lower_payment_binder(sylex_semantics_v1::Binder::TheRef(minimal_reference()))
                .as_slice(),
            [sylex_core::CostComponent::Let(sylex_core::Let {
                dest: FIRST_PAYMENT_DEF,
                expr: sylex_core::Expr::Object(sylex_core::Reference::Reg(
                    sylex_core::RefId(0)
                )),
            })]
        );
    }

    #[test]
    fn lowers_binder_choose_one() {
        let lowered = lower_payment_binder(sylex_semantics_v1::Binder::ChooseOne {
            filter: minimal_predicate(),
            by: minimal_reference(),
        });
        let [sylex_core::CostComponent::Choose(choice)] = lowered.as_slice() else {
            panic!("a payment-time single choice is one Choose instruction");
        };
        assert_eq!(choice.dest, FIRST_PAYMENT_DEF);
        assert_eq!(
            choice.by,
            sylex_core::Reference::Reg(sylex_core::RefId(0))
        );
        assert_eq!(
            choice.quantity,
            sylex_core::Quantity::Range(
                Some(sylex_core::Count::Literal(1)),
                Some(sylex_core::Count::Literal(1)),
            )
        );
        assert!(pins_minimal_filter(&choice.filter));
    }

    #[test]
    fn lowers_binder_produce() {
        let lowered = lower_payment_binder(sylex_semantics_v1::Binder::Produce(
            std::sync::Arc::new(sylex_semantics_v1::Action::Move(
                sylex_semantics_v1::Reference::This,
                sylex_semantics_v1::Destination::Zone(sylex_semantics_v1::Zone::Exile),
                [].into(),
                None,
            )),
        ));
        let [sylex_core::CostComponent::Act { dest, action }] = lowered.as_slice() else {
            panic!("a producing payment is one runnable action cost");
        };
        assert_eq!(
            *dest,
            Some(FIRST_PAYMENT_DEF),
            "the paid product is a definition the ability body reads ([CR#400.7])"
        );
        assert_eq!(
            **action,
            sylex_core::Action::Move(
                sylex_core::Reference::Reg(sylex_core::RefId(0)),
                sylex_core::Destination::Zone(sylex_core::Zone::Exile),
                [].into(),
                None,
            )
        );
    }

    #[test]
    fn lowers_binder_search_one() {
        let lowered = lower_payment_binder(sylex_semantics_v1::Binder::SearchOne {
            filter: minimal_predicate(),
            by: minimal_reference(),
            whose: minimal_reference(),
            from: [].into(),
            if_none: None,
        });
        let [sylex_core::CostComponent::Search(search)] = lowered.as_slice() else {
            panic!("a payment-time single search is one Search instruction");
        };
        assert_eq!(search.dest, FIRST_PAYMENT_DEF);
        assert_eq!(
            search.by,
            sylex_core::Reference::Reg(sylex_core::RefId(0))
        );
        assert_eq!(
            search.whose,
            sylex_core::Reference::Reg(sylex_core::RefId(0))
        );
        assert_eq!(
            search.quantity,
            sylex_core::Quantity::Range(
                Some(sylex_core::Count::Literal(1)),
                Some(sylex_core::Count::Literal(1)),
            )
        );
        assert!(search.from.is_empty());
        assert!(search.if_none.is_empty());
        assert!(pins_minimal_filter(&search.filter));
    }

    #[test]
    fn lowers_binder_choose() {
        let lowered = lower_payment_binder(sylex_semantics_v1::Binder::Choose {
            quantity: minimal_quantity(),
            filter: minimal_predicate(),
            by: minimal_reference(),
        });
        let [sylex_core::CostComponent::Choose(choice)] = lowered.as_slice() else {
            panic!("a payment-time plural choice is one Choose instruction");
        };
        assert_eq!(choice.dest, FIRST_PAYMENT_DEF);
        assert_eq!(
            choice.by,
            sylex_core::Reference::Reg(sylex_core::RefId(0))
        );
        assert_eq!(choice.quantity, sylex_core::Quantity::Range(None, None));
        assert!(pins_minimal_filter(&choice.filter));
    }

    #[test]
    fn lowers_binder_existing() {
        let lowered =
            lower_payment_binder(sylex_semantics_v1::Binder::Existing(minimal_selection()));
        let [sylex_core::CostComponent::Let(sylex_core::Let { dest, expr })] =
            lowered.as_slice()
        else {
            panic!("an existing group is pinned by one Let instruction");
        };
        assert_eq!(*dest, FIRST_PAYMENT_DEF);
        let sylex_core::Expr::Objects(sylex_core::Selection::SelectAll(filter)) = expr
        else {
            panic!("the pinned expression is the authored group query");
        };
        assert!(pins_minimal_filter(filter));
    }

    #[test]
    fn lowers_binder_search() {
        let lowered = lower_payment_binder(sylex_semantics_v1::Binder::Search {
            quantity: minimal_quantity(),
            filter: minimal_predicate(),
            by: minimal_reference(),
            whose: minimal_reference(),
            from: [].into(),
            if_none: None,
        });
        let [sylex_core::CostComponent::Search(search)] = lowered.as_slice() else {
            panic!("a payment-time plural search is one Search instruction");
        };
        assert_eq!(search.dest, FIRST_PAYMENT_DEF);
        assert_eq!(
            search.by,
            sylex_core::Reference::Reg(sylex_core::RefId(0))
        );
        assert_eq!(
            search.whose,
            sylex_core::Reference::Reg(sylex_core::RefId(0))
        );
        assert_eq!(search.quantity, sylex_core::Quantity::Range(None, None));
        assert!(search.from.is_empty());
        assert!(search.if_none.is_empty());
        assert!(pins_minimal_filter(&search.filter));
    }

    /// A macro invocation's provenance does not cross `lower`: the wrapper
    /// erases and the wrapped binder's own image is what lands.
    #[test]
    fn lowers_binder_expanded() {
        assert_eq!(
            lower_payment_binder(sylex_semantics_v1::Binder::Expanded(
                macro_ron::Expansion {
                    name: "X".into(),
                    args: macro_ron::ExpansionArgs::none(),
                    template: None,
                    value: Box::new(minimal_binder()),
                }
            ))
            .as_slice(),
            [sylex_core::CostComponent::Let(sylex_core::Let {
                dest: FIRST_PAYMENT_DEF,
                expr: sylex_core::Expr::Object(sylex_core::Reference::Reg(
                    sylex_core::RefId(0)
                )),
            })]
        );
    }

    #[test]
    fn lowers_cost_component_expanded() {
        let (lowered, _) =
            super::lower_cost_block(&[sylex_semantics_v1::CostComponent::Expanded(
                macro_ron::Expansion {
                    name: "X".into(),
                    args: macro_ron::ExpansionArgs::none(),
                    template: None,
                    value: Box::new(minimal_cost_component()),
                },
            )]);
        assert_matches!(lowered[0], sylex_core::CostComponent::Mana(_));
    }

    #[test]
    fn lowers_cost() {
        assert_matches!(
            sylex_semantics_v1::Cost([].into()).lower(),
            sylex_core::Cost(_)
        );
    }

    #[test]
    fn lowers_cost_tag() {
        assert_matches!(
            sylex_semantics_v1::CostTag("X".into()).lower(),
            sylex_core::CostTag(_)
        );
    }

    #[test]
    fn lowers_optional_cost() {
        assert_matches!(
            sylex_semantics_v1::OptionalCost {
                components: [].into(),
                tag: minimal_cost_tag(),
                repeatable: false
            }
            .lower(),
            sylex_core::OptionalCost {
                components: _,
                tag: sylex_core::CostTag(_),
                repeatable: false
            }
        );
    }

    #[test]
    fn lowers_total_cost() {
        assert_matches!(
            sylex_semantics_v1::TotalCost {
                base: [].into(),
                trace: [].into(),
                locked: false
            }
            .lower(),
            sylex_core::TotalCost {
                base: _,
                trace: _,
                locked: false
            }
        );
    }

    /// A chooser embedded in a keyword action is LIFTED into cost position:
    /// the decision becomes its own cost instruction, made before the verb
    /// that spends it ([CR#601.2b,701.9]), and the runnable action retains no
    /// binder. Re-spelled from
    /// `lifts_action_embedded_choice_into_cost_position`
    /// — `CostComponent::ChooseAndPay` and the `Binder` enum left core, so the
    /// lift's image is the instruction pair.
    #[test]
    fn lifts_action_embedded_choice_into_cost_position() {
        use sylex_semantics_v1::Action;
        use sylex_semantics_v1::OneShotEffect;
        use sylex_semantics_v1::With;

        let semantic =
            sylex_semantics_v1::CostComponent::Do(std::sync::Arc::new(Action::Composite {
                name: "Discard".into(),
                body: std::sync::Arc::new(OneShotEffect::With(With {
                    binder: sylex_semantics_v1::Binder::ChooseOne {
                        filter: sylex_semantics_v1::Predicate::Any,
                        by: sylex_semantics_v1::Reference::You,
                    },
                    body: std::sync::Arc::new(OneShotEffect::Act(Action::discard_what(
                        sylex_semantics_v1::Reference::That(sylex_semantics_v1::Sort::Card),
                    ))),
                })),
            }));

        let (_, (lowered, _)) =
            crate::region::in_region(crate::region::RegionKind::Activated, 0, || {
                super::lower_cost_block(&[semantic])
            });
        let [
            sylex_core::CostComponent::Choose(choice),
            sylex_core::CostComponent::Act { action, .. },
        ] = lowered.as_ref()
        else {
            panic!("the lifted choice precedes the one runnable action cost");
        };
        let sylex_core::Action::Composite { body, .. } = action.as_ref() else {
            panic!("lifting must retain the authored keyword-action boundary");
        };
        assert!(
            !body_carries_a_binder(body),
            "the runnable action must not retain the lifted choice"
        );
        assert!(
            reads_register(body, choice.dest.into()),
            "the runnable action pays through the register the choice wrote"
        );
    }

    /// A random payment subject is lifted the same way: the sample is its own
    /// cost instruction and the runnable action reads its register
    /// ([CR#601.2b]). Re-spelled from
    /// `lifts_action_embedded_random_selection_into_cost_position`.
    #[test]
    fn lifts_action_embedded_random_selection_into_cost_position() {
        let semantic = sylex_semantics_v1::CostComponent::Do(std::sync::Arc::new(
            sylex_semantics_v1::Action::discard(
                sylex_semantics_v1::Reference::You,
                sylex_semantics_v1::Count::Literal(2),
                true,
            ),
        ));
        let (_, (lowered, _)) =
            crate::region::in_region(crate::region::RegionKind::Activated, 0, || {
                super::lower_cost_block(&[semantic])
            });
        let [first, sylex_core::CostComponent::Act { action, .. }] = lowered.as_ref() else {
            panic!("the sampled subject precedes the one runnable action cost");
        };
        let sampled = match first {
            sylex_core::CostComponent::Sample(sylex_core::Sample {
                dest, filter, ..
            }) => {
                assert!(matches!(
                    filter.params[0].provenance,
                    sylex_core::Provenance::Candidate(_)
                ));
                assert!(
                    filter.params.iter().any(|param| {
                        param.provenance == sylex_core::Provenance::Controller
                    }),
                    "sample filter params: {:?}",
                    filter.params,
                );
                *dest
            }
            other => {
                panic!("an action-embedded random subject lowers into cost position: {other:?}")
            }
        };
        let sylex_core::Action::Composite { body, .. } = action.as_ref() else {
            panic!("lowering retains the authored discard action boundary");
        };
        assert!(
            !body_carries_a_binder(body),
            "the runnable action retains no unresolved random selection"
        );
        assert!(
            reads_register(body, sampled.into()),
            "the runnable action spends the sampled register"
        );
    }

    /// Whether a lowered instruction tree still carries a resolution-time
    /// binding decision — the thing lifting into cost position removes.
    fn body_carries_a_binder(body: &sylex_core::Instruction) -> bool {
        sylex_core::ron::options()
            .to_string(body)
            .expect("a lowered instruction tree serializes")
            .contains("Choose(")
    }

    /// Whether a lowered instruction tree reads `register` anywhere.
    fn reads_register(body: &sylex_core::Instruction, register: sylex_core::RefId) -> bool {
        sylex_core::ron::options()
            .to_string(body)
            .expect("a lowered instruction tree serializes")
            .contains(&format!("Reg({})", register.0))
    }
}
