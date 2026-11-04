//! `designation` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::DesignationScope {
    type Target = sylex_core::DesignationScope;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object => sylex_core::DesignationScope::Object,
            Self::Player => sylex_core::DesignationScope::Player,
            Self::Game => sylex_core::DesignationScope::Game,
        }
    }
}

impl Lower for sylex_semantics_v1::DesignationShape {
    type Target = sylex_core::DesignationShape;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Flag => sylex_core::DesignationShape::Flag,
            Self::Number => sylex_core::DesignationShape::Number,
            Self::Enum(f0) => sylex_core::DesignationShape::Enum(f0.lower()),
            Self::Relation => sylex_core::DesignationShape::Relation,
        }
    }
}

impl Lower for sylex_semantics_v1::DesignationUniqueness {
    type Target = sylex_core::DesignationUniqueness;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::None => sylex_core::DesignationUniqueness::None,
            Self::PerPlayer => sylex_core::DesignationUniqueness::PerPlayer,
            Self::PerGame => sylex_core::DesignationUniqueness::PerGame,
        }
    }
}

impl Lower for sylex_semantics_v1::DesignationPersistence {
    type Target = sylex_core::DesignationPersistence;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ObjectLifetime => sylex_core::DesignationPersistence::ObjectLifetime,
            Self::UntilEndOfTurn => sylex_core::DesignationPersistence::UntilEndOfTurn,
            Self::EffectSupplied => sylex_core::DesignationPersistence::EffectSupplied,
            Self::Permanently => sylex_core::DesignationPersistence::Permanently,
        }
    }
}

impl Lower for sylex_semantics_v1::DesignationDef {
    type Target = sylex_core::DesignationDef;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Stored {
                scope,
                shape,
                uniqueness,
                persistence,
                payload,
            } => sylex_core::DesignationDef::Stored {
                scope: scope.lower(),
                shape: shape.lower(),
                uniqueness: uniqueness.lower(),
                persistence: persistence.lower(),
                payload: payload.lower(),
            },
            Self::Derived(f0) => sylex_core::DesignationDef::Derived(f0.lower()),
            Self::DerivedIf(f0) => sylex_core::DesignationDef::DerivedIf(f0.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::CoreDeed {
    type Target = sylex_core::CoreDeed;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Attack => sylex_core::CoreDeed::Attack,
            Self::Block => sylex_core::CoreDeed::Block,
            Self::Target => sylex_core::CoreDeed::Target,
            Self::Copy => sylex_core::CoreDeed::Copy,
            Self::Draw => sylex_core::CoreDeed::Draw,
            Self::GainLife => sylex_core::CoreDeed::GainLife,
            Self::LoseGame => sylex_core::CoreDeed::LoseGame,
            Self::WinGame => sylex_core::CoreDeed::WinGame,
            Self::Spend => sylex_core::CoreDeed::Spend,
            Self::Trigger => sylex_core::CoreDeed::Trigger,
            Self::Put => sylex_core::CoreDeed::Put,
            Self::Return => sylex_core::CoreDeed::Return,
            Self::GainControl => sylex_core::CoreDeed::GainControl,
            Self::Unlock => sylex_core::CoreDeed::Unlock,
            Self::FullyUnlock => sylex_core::CoreDeed::FullyUnlock,
        }
    }
}

impl Lower for sylex_semantics_v1::DesignationDecl {
    type Target = sylex_core::DesignationDecl;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::DesignationDecl {
            name: self.name.lower(),
            definition: self.definition.lower(),
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
    fn lowers_designation_scope_object() {
        assert_matches!(
            sylex_semantics_v1::DesignationScope::Object.lower(),
            sylex_core::DesignationScope::Object
        );
    }

    #[test]
    fn lowers_designation_scope_player() {
        assert_matches!(
            sylex_semantics_v1::DesignationScope::Player.lower(),
            sylex_core::DesignationScope::Player
        );
    }

    #[test]
    fn lowers_designation_scope_game() {
        assert_matches!(
            sylex_semantics_v1::DesignationScope::Game.lower(),
            sylex_core::DesignationScope::Game
        );
    }

    #[test]
    fn lowers_designation_shape_flag() {
        assert_matches!(
            sylex_semantics_v1::DesignationShape::Flag.lower(),
            sylex_core::DesignationShape::Flag
        );
    }

    #[test]
    fn lowers_designation_shape_number() {
        assert_matches!(
            sylex_semantics_v1::DesignationShape::Number.lower(),
            sylex_core::DesignationShape::Number
        );
    }

    #[test]
    fn lowers_designation_shape_enum() {
        assert_matches!(
            sylex_semantics_v1::DesignationShape::Enum([].into()).lower(),
            sylex_core::DesignationShape::Enum(_)
        );
    }

    #[test]
    fn lowers_designation_shape_relation() {
        assert_matches!(
            sylex_semantics_v1::DesignationShape::Relation.lower(),
            sylex_core::DesignationShape::Relation
        );
    }

    #[test]
    fn lowers_designation_uniqueness_none() {
        assert_matches!(
            sylex_semantics_v1::DesignationUniqueness::None.lower(),
            sylex_core::DesignationUniqueness::None
        );
    }

    #[test]
    fn lowers_designation_uniqueness_per_player() {
        assert_matches!(
            sylex_semantics_v1::DesignationUniqueness::PerPlayer.lower(),
            sylex_core::DesignationUniqueness::PerPlayer
        );
    }

    #[test]
    fn lowers_designation_uniqueness_per_game() {
        assert_matches!(
            sylex_semantics_v1::DesignationUniqueness::PerGame.lower(),
            sylex_core::DesignationUniqueness::PerGame
        );
    }

    #[test]
    fn lowers_designation_persistence_object_lifetime() {
        assert_matches!(
            sylex_semantics_v1::DesignationPersistence::ObjectLifetime.lower(),
            sylex_core::DesignationPersistence::ObjectLifetime
        );
    }

    #[test]
    fn lowers_designation_persistence_until_end_of_turn() {
        assert_matches!(
            sylex_semantics_v1::DesignationPersistence::UntilEndOfTurn.lower(),
            sylex_core::DesignationPersistence::UntilEndOfTurn
        );
    }

    #[test]
    fn lowers_designation_persistence_effect_supplied() {
        assert_matches!(
            sylex_semantics_v1::DesignationPersistence::EffectSupplied.lower(),
            sylex_core::DesignationPersistence::EffectSupplied
        );
    }

    #[test]
    fn lowers_designation_persistence_permanently() {
        assert_matches!(
            sylex_semantics_v1::DesignationPersistence::Permanently.lower(),
            sylex_core::DesignationPersistence::Permanently
        );
    }

    #[test]
    fn lowers_designation_def_stored() {
        assert_matches!(
            sylex_semantics_v1::DesignationDef::Stored {
                scope: minimal_designation_scope(),
                shape: minimal_designation_shape(),
                uniqueness: minimal_designation_uniqueness(),
                persistence: minimal_designation_persistence(),
                payload: [].into()
            }
            .lower(),
            sylex_core::DesignationDef::Stored {
                scope: sylex_core::DesignationScope::Object,
                shape: sylex_core::DesignationShape::Flag,
                uniqueness: sylex_core::DesignationUniqueness::None,
                persistence: sylex_core::DesignationPersistence::ObjectLifetime,
                payload: _
            }
        );
    }

    #[test]
    fn lowers_designation_def_derived() {
        assert_matches!(
            sylex_semantics_v1::DesignationDef::Derived(minimal_predicate()).lower(),
            sylex_core::DesignationDef::Derived(sylex_core::Predicate::Class(
                sylex_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_designation_def_derived_if() {
        assert_matches!(
            sylex_semantics_v1::DesignationDef::DerivedIf(std::sync::Arc::new(minimal_condition()))
                .lower(),
            sylex_core::DesignationDef::DerivedIf(_)
        );
    }

    #[test]
    fn lowers_designation_decl() {
        assert_matches!(
            sylex_semantics_v1::DesignationDecl {
                name: "X".into(),
                definition: minimal_designation_def()
            }
            .lower(),
            sylex_core::DesignationDecl {
                name: _,
                definition: sylex_core::DesignationDef::Stored {
                    scope: sylex_core::DesignationScope::Object,
                    shape: sylex_core::DesignationShape::Flag,
                    uniqueness: sylex_core::DesignationUniqueness::None,
                    persistence: sylex_core::DesignationPersistence::ObjectLifetime,
                    payload: _
                }
            }
        );
    }
}
