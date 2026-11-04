//! `type` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for sylex_semantics_v1::Type {
    type Target = sylex_core::Type;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Artifact => sylex_core::Type::Artifact,
            Self::Battle => sylex_core::Type::Battle,
            Self::Creature => sylex_core::Type::Creature,
            Self::Dungeon => sylex_core::Type::Dungeon,
            Self::Enchantment => sylex_core::Type::Enchantment,
            Self::Instant => sylex_core::Type::Instant,
            Self::Kindred => sylex_core::Type::Kindred,
            Self::Land => sylex_core::Type::Land,
            Self::Planeswalker => sylex_core::Type::Planeswalker,
            Self::Sorcery => sylex_core::Type::Sorcery,
        }
    }
}

impl Lower for sylex_semantics_v1::Supertype {
    type Target = sylex_core::Supertype;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Basic => sylex_core::Supertype::Basic,
            Self::Legendary => sylex_core::Supertype::Legendary,
            Self::Ongoing => sylex_core::Supertype::Ongoing,
            Self::Snow => sylex_core::Supertype::Snow,
            Self::World => sylex_core::Supertype::World,
        }
    }
}

impl Lower for sylex_semantics_v1::Subtype {
    type Target = sylex_core::Subtype;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::Subtype {
            name: self.name.lower(),
            types: self.types.lower(),
            confers: crate::property::conferrals(self.confers.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::TypeDef {
    type Target = sylex_core::TypeDef;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::TypeDef {
            name: self.name.lower(),
            permanent_type: self.permanent.lower(),
            confers: crate::property::conferrals(self.confers.lower()),
        }
    }
}

impl Lower for sylex_semantics_v1::TypeRef {
    type Target = sylex_core::TypeRef;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::TypeRef(self.0.lower())
    }
}

impl Lower for sylex_semantics_v1::SubtypeRef {
    type Target = sylex_core::SubtypeRef;
    fn lower(self) -> <Self as Lower>::Target {
        sylex_core::SubtypeRef(self.0.lower())
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
    fn lowers_type_artifact() {
        assert_matches!(
            sylex_semantics_v1::Type::Artifact.lower(),
            sylex_core::Type::Artifact
        );
    }

    #[test]
    fn lowers_type_battle() {
        assert_matches!(
            sylex_semantics_v1::Type::Battle.lower(),
            sylex_core::Type::Battle
        );
    }

    #[test]
    fn lowers_type_creature() {
        assert_matches!(
            sylex_semantics_v1::Type::Creature.lower(),
            sylex_core::Type::Creature
        );
    }

    #[test]
    fn lowers_type_dungeon() {
        assert_matches!(
            sylex_semantics_v1::Type::Dungeon.lower(),
            sylex_core::Type::Dungeon
        );
    }

    #[test]
    fn lowers_type_enchantment() {
        assert_matches!(
            sylex_semantics_v1::Type::Enchantment.lower(),
            sylex_core::Type::Enchantment
        );
    }

    #[test]
    fn lowers_type_instant() {
        assert_matches!(
            sylex_semantics_v1::Type::Instant.lower(),
            sylex_core::Type::Instant
        );
    }

    #[test]
    fn lowers_type_kindred() {
        assert_matches!(
            sylex_semantics_v1::Type::Kindred.lower(),
            sylex_core::Type::Kindred
        );
    }

    #[test]
    fn lowers_type_land() {
        assert_matches!(
            sylex_semantics_v1::Type::Land.lower(),
            sylex_core::Type::Land
        );
    }

    #[test]
    fn lowers_type_planeswalker() {
        assert_matches!(
            sylex_semantics_v1::Type::Planeswalker.lower(),
            sylex_core::Type::Planeswalker
        );
    }

    #[test]
    fn lowers_type_sorcery() {
        assert_matches!(
            sylex_semantics_v1::Type::Sorcery.lower(),
            sylex_core::Type::Sorcery
        );
    }

    #[test]
    fn lowers_supertype_basic() {
        assert_matches!(
            sylex_semantics_v1::Supertype::Basic.lower(),
            sylex_core::Supertype::Basic
        );
    }

    #[test]
    fn lowers_supertype_legendary() {
        assert_matches!(
            sylex_semantics_v1::Supertype::Legendary.lower(),
            sylex_core::Supertype::Legendary
        );
    }

    #[test]
    fn lowers_supertype_ongoing() {
        assert_matches!(
            sylex_semantics_v1::Supertype::Ongoing.lower(),
            sylex_core::Supertype::Ongoing
        );
    }

    #[test]
    fn lowers_supertype_snow() {
        assert_matches!(
            sylex_semantics_v1::Supertype::Snow.lower(),
            sylex_core::Supertype::Snow
        );
    }

    #[test]
    fn lowers_supertype_world() {
        assert_matches!(
            sylex_semantics_v1::Supertype::World.lower(),
            sylex_core::Supertype::World
        );
    }

    #[test]
    fn lowers_subtype() {
        assert_matches!(
            sylex_semantics_v1::Subtype {
                name: "X".into(),
                types: [].into(),
                confers: [].into()
            }
            .lower(),
            sylex_core::Subtype {
                name: _,
                types: _,
                confers: _
            }
        );
    }

    #[test]
    fn lowers_type_def() {
        assert_matches!(
            sylex_semantics_v1::TypeDef {
                name: "X".into(),
                permanent: false,
                confers: [].into()
            }
            .lower(),
            sylex_core::TypeDef {
                name: _,
                permanent_type: false,
                confers: _
            }
        );
    }

    #[test]
    fn lowers_type_ref() {
        assert_matches!(
            sylex_semantics_v1::TypeRef(std::sync::Arc::new(minimal_type_def())).lower(),
            sylex_core::TypeRef(_)
        );
    }

    #[test]
    fn lowers_subtype_ref() {
        assert_matches!(
            sylex_semantics_v1::SubtypeRef(std::sync::Arc::new(minimal_subtype())).lower(),
            sylex_core::SubtypeRef(_)
        );
    }
}
