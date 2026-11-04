//! `selection` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::Selection {
    type Target = sylex_core::Selection;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SelectAll(f0) => sylex_core::Selection::SelectAll(std::sync::Arc::new(
                crate::region::predicate_region(|| f0.lower()),
            )),
            Self::Union(f0) => sylex_core::Selection::Union(f0.lower()),
            Self::InChosenOrder(f0, f1) => {
                sylex_core::Selection::InChosenOrder(f0.lower(), f1.lower())
            }
            Self::Random(f0, f1) => sylex_core::Selection::Random(
                f0.lower(),
                std::sync::Arc::new(crate::region::predicate_region(|| f1.lower())),
            ),
            // [CR#607.1]: "them" over a noted group — a register of this
            // region, or the card's memory cell read through the ability's
            // declared `Provenance::Linked` parameter (ADR law 8).
            Self::AmongNoted(name, _) => crate::region::group_read(
                crate::region::named(&name)
                    .or_else(|| crate::region::cell_read(&name))
                    .unwrap_or_else(|| {
                        crate::region::refuse(&format!(
                            "noted selection `{name}` has no declared cell — no ability on this \
                             card writes it ([CR#607.1])"
                        ));
                        sylex_core::RefId(0)
                    }),
            ),
            Self::TopOfLibrary { count, whose } => sylex_core::Selection::TopOfLibrary {
                count: count.lower(),
                whose: whose.lower(),
            },
            Self::BottomOfLibrary { count, whose } => sylex_core::Selection::BottomOfLibrary {
                count: count.lower(),
                whose: whose.lower(),
            },
            Self::LibraryOf(f0) => sylex_core::Selection::LibraryOf(f0.lower()),
            Self::TopOfGraveyard { count, of } => sylex_core::Selection::TopOfGraveyard {
                count: count.lower(),
                of: of.lower(),
            },
            Self::Targets(index) => crate::region::target(index).map_or(
                sylex_core::Selection::Reg(sylex_core::RefId(
                    6 + u32::try_from(index).expect("target index fits u32"),
                )),
                sylex_core::Selection::Reg,
            ),
            Self::ValidTargetsFor(f0) => sylex_core::Selection::ValidTargetsFor(f0.lower()),
            Self::They => crate::region::group_read(
                crate::region::they(None).expect("unbound `They` during semantic lowering"),
            ),
            // The mention's own sort fixes which collection domain the
            // register read belongs to ([CR#700.3b] — those PILES are not a
            // group of Entities).
            Self::Them(sort) => {
                let register =
                    crate::region::they(Some(sort)).expect("unbound sorted plural during lowering");
                if sort == sylex_semantics_v1::Sort::Pile {
                    sylex_core::Selection::Pile(register)
                } else {
                    crate::region::group_read(register)
                }
            }
            Self::PilesOf { .. } => {
                crate::region::refuse(
                    "noted pile sets have no register spelling yet; owner: engine-piles",
                );
                sylex_core::Selection::Union([].into())
            }
            Self::Pick { op, proj } => sylex_core::Selection::Pick {
                op: op.lower(),
                proj: proj.lower(),
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
    fn lowers_selection_select_all() {
        assert_matches!(
            sylex_semantics_v1::Selection::SelectAll(minimal_predicate()).lower(),
            sylex_core::Selection::SelectAll(_)
        );
    }

    #[test]
    fn lowers_selection_union() {
        assert_matches!(
            sylex_semantics_v1::Selection::Union(Vec::new()).lower(),
            sylex_core::Selection::Union(_)
        );
    }

    #[test]
    fn lowers_selection_in_chosen_order() {
        assert_matches!(
            sylex_semantics_v1::Selection::InChosenOrder(
                std::sync::Arc::new(minimal_selection()),
                minimal_reference()
            )
            .lower(),
            sylex_core::Selection::InChosenOrder(
                _,
                sylex_core::Reference::Reg(sylex_core::RefId(0))
            )
        );
    }

    /// Re-spelled for the candidate region `Random` now carries: same
    /// quantity, same lowered predicate, plus the declared domain.
    #[test]
    fn lowers_selection_random() {
        let lowered =
            sylex_semantics_v1::Selection::Random(minimal_quantity(), minimal_predicate()).lower();
        let sylex_core::Selection::Random(quantity, region) = lowered else {
            panic!("expected a Random selection");
        };
        assert_eq!(quantity, sylex_core::Quantity::Range(None, None));
        assert_eq!(
            region.body,
            sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
        );
        assert_eq!(region.candidate_domain(), sylex_core::Domain::Object);
    }

    #[test]
    fn lowers_selection_top_of_library() {
        assert_matches!(
            sylex_semantics_v1::Selection::TopOfLibrary {
                count: minimal_count(),
                whose: minimal_reference()
            }
            .lower(),
            sylex_core::Selection::TopOfLibrary {
                count: sylex_core::Count::Literal(0),
                whose: sylex_core::Reference::Reg(sylex_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_selection_bottom_of_library() {
        assert_matches!(
            sylex_semantics_v1::Selection::BottomOfLibrary {
                count: minimal_count(),
                whose: minimal_reference()
            }
            .lower(),
            sylex_core::Selection::BottomOfLibrary {
                count: sylex_core::Count::Literal(0),
                whose: sylex_core::Reference::Reg(sylex_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_selection_library_of() {
        assert_matches!(
            sylex_semantics_v1::Selection::LibraryOf(minimal_reference()).lower(),
            sylex_core::Selection::LibraryOf(sylex_core::Reference::Reg(
                sylex_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_selection_top_of_graveyard() {
        assert_matches!(
            sylex_semantics_v1::Selection::TopOfGraveyard {
                count: minimal_count(),
                of: minimal_reference()
            }
            .lower(),
            sylex_core::Selection::TopOfGraveyard {
                count: sylex_core::Count::Literal(0),
                of: sylex_core::Reference::Reg(sylex_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_selection_targets() {
        assert_matches!(
            sylex_semantics_v1::Selection::Targets(0).lower(),
            sylex_core::Selection::Reg(sylex_core::RefId(6))
        );
    }

    #[test]
    fn lowers_selection_valid_targets_for() {
        assert_matches!(
            sylex_semantics_v1::Selection::ValidTargetsFor(minimal_reference()).lower(),
            sylex_core::Selection::ValidTargetsFor(sylex_core::Reference::Reg(
                sylex_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_selection_piles_of() {
        let error = crate::lower_for_test(|| {
            sylex_semantics_v1::Selection::PilesOf {
                note: "X".into(),
                of: minimal_reference(),
            }
            .lower()
        })
        .expect_err("an unimplemented noted pile set is refused");
        assert_eq!(&*error.card, "Lowering Test");
        assert!(
            error
                .message
                .contains("noted pile sets have no register spelling yet"),
            "the returned diagnostic carries the refusal, got {:?}",
            error.message
        );
    }

    #[test]
    fn lowers_selection_pick() {
        assert_matches!(
            sylex_semantics_v1::Selection::Pick {
                op: minimal_aggregate_op(),
                proj: minimal_projection()
            }
            .lower(),
            sylex_core::Selection::Pick {
                op: sylex_core::AggregateOp::SumOf,
                proj: sylex_core::Projection {
                    of: sylex_core::Countable::Objects(_),
                    by: _
                }
            }
        );
    }

    #[test]
    fn lowers_selection_expanded() {
        assert_matches!(
            sylex_semantics_v1::Selection::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_selection())
            })
            .lower(),
            sylex_core::Selection::SelectAll(_)
        );
    }

    /// A plural anaphor ("they") reads the register holding the group.
    /// Re-spelled from `lowers_selection_they`: `Selection::They` left core
    /// with the discourse channel.
    #[test]
    fn lowers_selection_they_to_the_group_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let group = crate::region::define(sylex_core::Kind::Entities);
            crate::region::push_antecedent(
                group.into(),
                sylex_core::Kind::Entities,
                crate::region::Cardinality::Many,
                None,
                crate::region::Site::ExecutionFrame,
            );
            sylex_semantics_v1::Selection::They.lower()
        });
        assert_eq!(
            lowered,
            sylex_core::Selection::Reg(sylex_core::RefId(3))
        );
    }

    /// A sorted plural anaphor ("those players") picks the group register
    /// whose sort it matches. Re-spelled from `lowers_selection_them`.
    #[test]
    fn lowers_selection_them_to_the_sorted_group_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let cards = crate::region::define(sylex_core::Kind::Entities);
            crate::region::push_antecedent(
                cards.into(),
                sylex_core::Kind::Entities,
                crate::region::Cardinality::Many,
                Some(sylex_semantics_v1::Sort::Card),
                crate::region::Site::ExecutionFrame,
            );
            let players = crate::region::define(sylex_core::Kind::Entities);
            crate::region::push_antecedent(
                players.into(),
                sylex_core::Kind::Entities,
                crate::region::Cardinality::Many,
                Some(sylex_semantics_v1::Sort::Player),
                crate::region::Site::ExecutionFrame,
            );
            sylex_semantics_v1::Selection::Them(sylex_semantics_v1::Sort::Player).lower()
        });
        assert_eq!(
            lowered,
            sylex_core::Selection::Reg(sylex_core::RefId(4)),
            "the sort picks the player group, not the nearer card one"
        );
    }

    /// "Among the cards noted this way" reads the register the noting
    /// instruction defined ([CR#607.2a]) — core keeps no name-keyed store.
    /// Re-spelled from `lowers_selection_among_noted`.
    #[test]
    fn lowers_selection_among_noted_to_the_noted_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            crate::region::bind_named("X".into(), sylex_core::RefId(2));
            sylex_semantics_v1::Selection::AmongNoted(
                "X".into(),
                sylex_semantics_v1::Quantity::one(),
            )
            .lower()
        });
        assert_eq!(
            lowered,
            sylex_core::Selection::Reg(sylex_core::RefId(2))
        );
    }
}
