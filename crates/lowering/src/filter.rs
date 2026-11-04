//! `filter` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

/// The core predicate for semantics' exclusive object-kind axis
/// ([CR#109.1]). `Player` is an Entity-level classification — a player is not
/// an object ([CR#102.1]) — and every other member names one CR object class,
/// which core tests independently of the rest.
#[must_use]
pub fn object_kind_predicate(kind: sylex_semantics_v1::ObjectKind) -> sylex_core::Predicate {
    use sylex_core::ObjectClass as C;
    use sylex_semantics_v1::ObjectKind as K;
    let class = match kind {
        K::Player => {
            return sylex_core::Predicate::Entity(sylex_core::EntityClass::Player);
        }
        K::Ability => C::AbilityOnStack,
        K::Card => C::Card,
        K::CardCopy => C::CopyOfACard,
        K::Emblem => C::Emblem,
        K::Spell => C::Spell,
        K::Token => C::Token,
    };
    sylex_core::Predicate::Class(class)
}

impl Lower for sylex_semantics_v1::CharacteristicPredicate {
    type Target = sylex_core::CharacteristicPredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Type(f0) => sylex_core::CharacteristicPredicate::Type(f0.lower()),
            Self::Subtype(f0) => sylex_core::CharacteristicPredicate::Subtype(f0.lower()),
            Self::Supertype(f0) => sylex_core::CharacteristicPredicate::Supertype(f0.lower()),
            Self::ColorIs(f0) => sylex_core::CharacteristicPredicate::ColorIs(f0.lower()),
            Self::Named(name) => crate::region::named(&name).map_or_else(
                || sylex_core::CharacteristicPredicate::Named(name.lower()),
                sylex_core::CharacteristicPredicate::NamedReg,
            ),
            Self::Stat(f0, f1, f2) => {
                sylex_core::CharacteristicPredicate::Stat(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Multicolored => sylex_core::CharacteristicPredicate::Multicolored,
            Self::Colorless => sylex_core::CharacteristicPredicate::Colorless,
            Self::Has(f0) => sylex_core::CharacteristicPredicate::Has(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::StatePredicate {
    type Target = sylex_core::StatePredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::InZone(f0) => sylex_core::StatePredicate::InZone(f0.lower()),
            Self::Status(f0) => sylex_core::StatePredicate::Status(f0.lower()),
            Self::SummoningSick => sylex_core::StatePredicate::SummoningSick,
            Self::HasCounter(f0) => sylex_core::StatePredicate::HasCounter(f0.lower()),
            Self::Designated(f0) => sylex_core::StatePredicate::Designated(f0.lower()),
            Self::RelatedBy(f0, f1) => {
                sylex_core::StatePredicate::RelatedBy(f0.lower(), f1.lower())
            }
            Self::Attacking => sylex_core::StatePredicate::Attacking,
            Self::Blocking => sylex_core::StatePredicate::Blocking,
            Self::Unblocked => sylex_core::StatePredicate::Unblocked,
            Self::Targets(f0) => sylex_core::StatePredicate::Targets(f0.lower()),
            Self::TargetCount(f0) => sylex_core::StatePredicate::TargetCount(f0.lower()),
            Self::WasPaidWith(f0) => sylex_core::StatePredicate::WasPaidWith(f0.lower()),
            Self::WasCastWith(f0) => sylex_core::StatePredicate::WasCastWith(f0.lower()),
            Self::WasPutFrom(f0) => sylex_core::StatePredicate::WasPutFrom(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::RelationPredicate {
    type Target = sylex_core::RelationPredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ControlledBy(f0) => sylex_core::RelationPredicate::ControlledBy(f0.lower()),
            Self::Controls(f0) => sylex_core::RelationPredicate::Controls(f0.lower()),
            Self::Owner(f0) => sylex_core::RelationPredicate::Owner(f0.lower()),
            Self::OpponentOf(f0) => sylex_core::RelationPredicate::OpponentOf(f0.lower()),
            Self::TeammateOf(f0) => sylex_core::RelationPredicate::TeammateOf(f0.lower()),
            Self::AttachedTo(f0) => sylex_core::RelationPredicate::AttachedTo(f0.lower()),
            Self::Attachment(f0) => sylex_core::RelationPredicate::Attachment(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::Adjacency {
    type Target = sylex_core::Adjacency;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Above => sylex_core::Adjacency::Above,
            Self::Below => sylex_core::Adjacency::Below,
        }
    }
}

impl Lower for sylex_semantics_v1::Predicate {
    type Target = sylex_core::Predicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Kind(f0) => object_kind_predicate(f0),
            Self::Characteristic(f0) => sylex_core::Predicate::Characteristic(f0.lower()),
            Self::State(f0) => sylex_core::Predicate::State(f0.lower()),
            Self::Relation(f0) => sylex_core::Predicate::Relation(f0.lower()),
            Self::Ref(f0) => sylex_core::Predicate::Ref(f0.lower()),
            Self::Adjacent(f0, f1) => sylex_core::Predicate::Adjacent(f0.lower(), f1.lower()),
            Self::PlayerStatCmp(f0, f1, f2) => {
                sylex_core::Predicate::PlayerStatCmp(f0.lower(), f1.lower(), f2.lower())
            }
            Self::FromSource(f0) => sylex_core::Predicate::FromSource(f0.lower()),
            Self::And(f0) => sylex_core::Predicate::And(f0.lower()),
            Self::Or(f0) => sylex_core::Predicate::Or(f0.lower()),
            Self::Not(f0) => sylex_core::Predicate::Not(f0.lower()),
            Self::Where(f0) => sylex_core::Predicate::Where(std::sync::Arc::new(
                crate::region::candidate_region(|| std::sync::Arc::unwrap_or_clone(f0).lower()),
            )),
            Self::Any => sylex_core::Predicate::Any,
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

    use super::object_kind_predicate;
    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_object_kind_ability() {
        assert_matches!(
            object_kind_predicate(sylex_semantics_v1::ObjectKind::Ability),
            sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
        );
    }

    #[test]
    fn lowers_object_kind_card() {
        assert_matches!(
            object_kind_predicate(sylex_semantics_v1::ObjectKind::Card),
            sylex_core::Predicate::Class(sylex_core::ObjectClass::Card)
        );
    }

    #[test]
    fn lowers_object_kind_card_copy() {
        assert_matches!(
            object_kind_predicate(sylex_semantics_v1::ObjectKind::CardCopy),
            sylex_core::Predicate::Class(sylex_core::ObjectClass::CopyOfACard)
        );
    }

    #[test]
    fn lowers_object_kind_emblem() {
        assert_matches!(
            object_kind_predicate(sylex_semantics_v1::ObjectKind::Emblem),
            sylex_core::Predicate::Class(sylex_core::ObjectClass::Emblem)
        );
    }

    #[test]
    fn lowers_object_kind_player() {
        assert_matches!(
            object_kind_predicate(sylex_semantics_v1::ObjectKind::Player),
            sylex_core::Predicate::Entity(sylex_core::EntityClass::Player)
        );
    }

    #[test]
    fn lowers_object_kind_spell() {
        assert_matches!(
            object_kind_predicate(sylex_semantics_v1::ObjectKind::Spell),
            sylex_core::Predicate::Class(sylex_core::ObjectClass::Spell)
        );
    }

    #[test]
    fn lowers_object_kind_token() {
        assert_matches!(
            object_kind_predicate(sylex_semantics_v1::ObjectKind::Token),
            sylex_core::Predicate::Class(sylex_core::ObjectClass::Token)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_type() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::Type(minimal_type_ref()).lower(),
            sylex_core::CharacteristicPredicate::Type(sylex_core::TypeRef(_))
        );
    }

    #[test]
    fn lowers_characteristic_predicate_subtype() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::Subtype(minimal_subtype_ref()).lower(),
            sylex_core::CharacteristicPredicate::Subtype(sylex_core::SubtypeRef(_))
        );
    }

    #[test]
    fn lowers_characteristic_predicate_supertype() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::Supertype(minimal_supertype()).lower(),
            sylex_core::CharacteristicPredicate::Supertype(sylex_core::Supertype::Basic)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_color_is() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::ColorIs(minimal_color()).lower(),
            sylex_core::CharacteristicPredicate::ColorIs(sylex_core::Color::White)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_named() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::Named("X".into()).lower(),
            sylex_core::CharacteristicPredicate::Named(_)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_stat() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::Stat(
                minimal_stat(),
                minimal_cmp(),
                minimal_count()
            )
            .lower(),
            sylex_core::CharacteristicPredicate::Stat(
                sylex_core::Stat::Power,
                sylex_core::Cmp::Eq,
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_characteristic_predicate_multicolored() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::Multicolored.lower(),
            sylex_core::CharacteristicPredicate::Multicolored
        );
    }

    #[test]
    fn lowers_characteristic_predicate_colorless() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::Colorless.lower(),
            sylex_core::CharacteristicPredicate::Colorless
        );
    }

    #[test]
    fn lowers_characteristic_predicate_has() {
        assert_matches!(
            sylex_semantics_v1::CharacteristicPredicate::Has(minimal_keyword_ref()).lower(),
            sylex_core::CharacteristicPredicate::Has(sylex_core::KeywordRef(_))
        );
    }

    #[test]
    fn lowers_state_predicate_in_zone() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::InZone(minimal_zone()).lower(),
            sylex_core::StatePredicate::InZone(sylex_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_state_predicate_status() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::Status(minimal_status()).lower(),
            sylex_core::StatePredicate::Status(sylex_core::Status::Tapped)
        );
    }

    #[test]
    fn lowers_state_predicate_summoning_sick() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::SummoningSick.lower(),
            sylex_core::StatePredicate::SummoningSick
        );
    }

    #[test]
    fn lowers_state_predicate_has_counter() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::HasCounter(minimal_counter_ref()).lower(),
            sylex_core::StatePredicate::HasCounter(sylex_core::CounterRef(_))
        );
    }

    #[test]
    fn lowers_state_predicate_designated() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::Designated("X".into()).lower(),
            sylex_core::StatePredicate::Designated(_)
        );
    }

    #[test]
    fn lowers_state_predicate_related_by() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::RelatedBy(
                "X".into(),
                std::sync::Arc::new(minimal_predicate())
            )
            .lower(),
            sylex_core::StatePredicate::RelatedBy(_, _)
        );
    }

    #[test]
    fn lowers_state_predicate_attacking() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::Attacking.lower(),
            sylex_core::StatePredicate::Attacking
        );
    }

    #[test]
    fn lowers_state_predicate_blocking() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::Blocking.lower(),
            sylex_core::StatePredicate::Blocking
        );
    }

    #[test]
    fn lowers_state_predicate_unblocked() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::Unblocked.lower(),
            sylex_core::StatePredicate::Unblocked
        );
    }

    #[test]
    fn lowers_state_predicate_targets() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::Targets(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            sylex_core::StatePredicate::Targets(_)
        );
    }

    #[test]
    fn lowers_state_predicate_target_count() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::TargetCount(minimal_count_bound()).lower(),
            sylex_core::StatePredicate::TargetCount(sylex_core::CountBound::Eq(
                sylex_core::Count::Literal(0)
            ))
        );
    }

    #[test]
    fn lowers_state_predicate_was_paid_with() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::WasPaidWith(minimal_cost_tag()).lower(),
            sylex_core::StatePredicate::WasPaidWith(sylex_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_state_predicate_was_cast_with() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::WasCastWith(minimal_cost_tag()).lower(),
            sylex_core::StatePredicate::WasCastWith(sylex_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_state_predicate_was_put_from() {
        assert_matches!(
            sylex_semantics_v1::StatePredicate::WasPutFrom(minimal_zone()).lower(),
            sylex_core::StatePredicate::WasPutFrom(sylex_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_relation_predicate_controlled_by() {
        assert_matches!(
            sylex_semantics_v1::RelationPredicate::ControlledBy(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            sylex_core::RelationPredicate::ControlledBy(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_controls() {
        assert_matches!(
            sylex_semantics_v1::RelationPredicate::Controls(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            sylex_core::RelationPredicate::Controls(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_owner() {
        assert_matches!(
            sylex_semantics_v1::RelationPredicate::Owner(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            sylex_core::RelationPredicate::Owner(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_opponent_of() {
        assert_matches!(
            sylex_semantics_v1::RelationPredicate::OpponentOf(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            sylex_core::RelationPredicate::OpponentOf(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_teammate_of() {
        assert_matches!(
            sylex_semantics_v1::RelationPredicate::TeammateOf(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            sylex_core::RelationPredicate::TeammateOf(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_attached_to() {
        assert_matches!(
            sylex_semantics_v1::RelationPredicate::AttachedTo(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            sylex_core::RelationPredicate::AttachedTo(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_attachment() {
        assert_matches!(
            sylex_semantics_v1::RelationPredicate::Attachment(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            sylex_core::RelationPredicate::Attachment(_)
        );
    }

    #[test]
    fn lowers_adjacency_above() {
        assert_matches!(
            sylex_semantics_v1::Adjacency::Above.lower(),
            sylex_core::Adjacency::Above
        );
    }

    #[test]
    fn lowers_adjacency_below() {
        assert_matches!(
            sylex_semantics_v1::Adjacency::Below.lower(),
            sylex_core::Adjacency::Below
        );
    }

    #[test]
    fn lowers_predicate_kind() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Kind(minimal_object_kind()).lower(),
            sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
        );
    }

    #[test]
    fn lowers_predicate_characteristic() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Characteristic(minimal_characteristic_predicate())
                .lower(),
            sylex_core::Predicate::Characteristic(
                sylex_core::CharacteristicPredicate::Type(sylex_core::TypeRef(_))
            )
        );
    }

    #[test]
    fn lowers_predicate_state() {
        assert_matches!(
            sylex_semantics_v1::Predicate::State(minimal_state_predicate()).lower(),
            sylex_core::Predicate::State(sylex_core::StatePredicate::InZone(
                sylex_core::Zone::Battlefield
            ))
        );
    }

    #[test]
    fn lowers_predicate_relation() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Relation(minimal_relation_predicate()).lower(),
            sylex_core::Predicate::Relation(sylex_core::RelationPredicate::ControlledBy(_))
        );
    }

    #[test]
    fn lowers_predicate_ref() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Ref(minimal_reference()).lower(),
            sylex_core::Predicate::Ref(sylex_core::Reference::Reg(sylex_core::RefId(
                0
            )))
        );
    }

    #[test]
    fn lowers_predicate_adjacent() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Adjacent(minimal_adjacency(), minimal_reference())
                .lower(),
            sylex_core::Predicate::Adjacent(
                sylex_core::Adjacency::Above,
                sylex_core::Reference::Reg(sylex_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_predicate_player_stat_cmp() {
        assert_matches!(
            sylex_semantics_v1::Predicate::PlayerStatCmp(
                minimal_player_attr(),
                minimal_cmp(),
                minimal_count()
            )
            .lower(),
            sylex_core::Predicate::PlayerStatCmp(
                sylex_core::PlayerAttr::Life,
                sylex_core::Cmp::Eq,
                sylex_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_predicate_from_source() {
        assert_matches!(
            sylex_semantics_v1::Predicate::FromSource(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            sylex_core::Predicate::FromSource(_)
        );
    }

    #[test]
    fn lowers_predicate_and() {
        assert_matches!(
            sylex_semantics_v1::Predicate::And([].into()).lower(),
            sylex_core::Predicate::And(_)
        );
    }

    #[test]
    fn lowers_predicate_or() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Or([].into()).lower(),
            sylex_core::Predicate::Or(_)
        );
    }

    #[test]
    fn lowers_predicate_not() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Not(std::sync::Arc::new(minimal_predicate())).lower(),
            sylex_core::Predicate::Not(_)
        );
    }

    #[test]
    fn lowers_predicate_where() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Where(std::sync::Arc::new(minimal_condition())).lower(),
            sylex_core::Predicate::Where(_)
        );
    }

    #[test]
    fn lowers_predicate_any() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Any.lower(),
            sylex_core::Predicate::Any
        );
    }

    #[test]
    fn lowers_predicate_expanded() {
        assert_matches!(
            sylex_semantics_v1::Predicate::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_predicate())
            })
            .lower(),
            sylex_core::Predicate::Class(sylex_core::ObjectClass::AbilityOnStack)
        );
    }
}
